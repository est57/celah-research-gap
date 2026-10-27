//! Words dropped from title and abstract counts: English and Indonesian function words,
//! generic research wording and publisher boilerplate.

use std::collections::HashSet;
use std::sync::OnceLock;

const ENGLISH: &[&str] = &[
    "about", "above", "after", "again", "against", "all", "also", "although", "among", "and", "any", "are", "around",
    "because", "been", "before", "being", "below", "between", "both", "but", "can", "could", "did", "does", "doing",
    "down", "due", "during", "each", "either", "etc", "even", "ever", "every", "few", "for", "from", "further", "had",
    "has", "have", "having", "her", "here", "hers", "herself", "him", "himself", "his", "how", "however", "into", "its",
    "itself", "just", "least", "less", "may", "might", "more", "most", "much", "must", "neither", "nor", "not", "now",
    "off", "often", "once", "only", "other", "others", "our", "ours", "out", "over", "own", "per", "rather", "same",
    "several", "shall", "she", "should", "since", "some", "such", "than", "that", "the", "their", "theirs", "them",
    "themselves", "then", "there", "therefore", "these", "they", "this", "those", "though", "through", "thus", "too",
    "under", "until", "upon", "very", "via", "was", "were", "what", "when", "where", "whether", "which", "while", "who",
    "whom", "whose", "why", "will", "with", "within", "without", "would", "yet", "you", "your", "yours",
];

const INDONESIAN: &[&str] = &[
    "ada", "adalah", "agar", "akan", "antara", "atau", "bagi", "bahwa", "beberapa", "belum", "dalam", "dan", "dapat",
    "dari", "dengan", "hal", "hanya", "harus", "ini", "itu", "juga", "karena", "ke", "kepada", "lebih", "maka",
    "masih", "melalui", "menjadi", "namun", "oleh", "pada", "para", "saat", "sangat", "secara", "sebagai", "sedangkan",
    "sehingga", "sejak", "serta", "setelah", "sudah", "tanpa", "telah", "tentang", "terhadap", "tersebut", "untuk",
    "yaitu", "yakni", "yang",
];

/// Wording that appears in nearly every abstract and says nothing about the topic.
const GENERIC: &[&str] = &[
    "abstract", "aim", "aims", "approach", "article", "based", "conclusion", "conclusions", "findings", "found",
    "furthermore", "moreover", "objective", "objectives", "paper", "present", "presented", "presents", "propose",
    "proposed", "purpose", "result", "results", "show", "showed", "shown", "shows", "studies", "study", "use", "used",
    "uses", "using", "well", "one", "two", "three", "new", "copyright", "elsevier", "ltd", "inc", "reserved",
    "rights", "springer", "wiley", "taylor", "francis", "published", "publisher", "licensee", "mdpi",
];

fn set() -> &'static HashSet<&'static str> {
    static SET: OnceLock<HashSet<&'static str>> = OnceLock::new();
    SET.get_or_init(|| ENGLISH.iter().chain(INDONESIAN).chain(GENERIC).copied().collect())
}

pub fn is_stop(word: &str) -> bool {
    set().contains(word)
}
