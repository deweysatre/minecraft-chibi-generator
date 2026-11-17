use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum View {
    /// Vue de face, de 3/4, regard vers la droite (valeur par défaut)
    Right34,
    /// Vue de face, de 3/4, regard vers la gauche
    Left34,
    /// Vue de face
    Front,
    /// Vue de dos
    Back,
    /// Vue du dessous
    Bottom,
    /// Vue du dessus
    Top,
    /// Vue de droite
    Right,
    /// Vue de gauche
    Left,
    /// Vue de dos, de 3/4, regard vers la droite
    BackRight34,
    /// Vue de dos, de 3/4, regard vers la gauche
    BackLeft34,

    /// Variante interne : toutes les vues sont sélectionnées via `--all-views`.
    #[value(skip)]
    All,
}

#[derive(Debug, Parser, Clone)]
#[command(
    name = "minecraft-chibi-generator",
    about = "Génère un avatar chibi Classic à partir d'un pseudo Minecraft Java"
)]
pub struct Args {
    /// Nom d'utilisateur Java
    #[arg(short, long, conflicts_with = "input", value_name = "username")]
    pub username: Option<String>,

    /// Charger le skin depuis un fichier
    #[arg(short, long, conflicts_with = "username", value_name = "path")]
    pub input: Option<PathBuf>,

    /// Chemin du fichier .png du résultat
    #[arg(short, long, value_name = "path")]
    pub output: Option<PathBuf>,

    /// Facteur entier d'agrandissement (par défaut : 1 = rendu natif 24x33).
    #[arg(
        short,
        long,
        value_name = "scale",
        value_parser = clap::value_parser!(u32).range(1..=64),
    )]
    pub scale: Option<u32>,

    /// La vue va être créée (par défaut : right 3/4).
    #[arg(
        short,
        long,
        value_enum,
        conflicts_with = "all_views",
        value_name = "view"
    )]
    pub view: Option<View>,

    /// Créer toutes les vues (doit recevoir un dossier).
    #[arg(short, long, value_name = "path", conflicts_with_all = ["output", "view"])]
    pub all_views: Option<PathBuf>,

    /// Ne pas trim les lignes transparentes
    #[arg(long)]
    pub no_trim: bool,
}
