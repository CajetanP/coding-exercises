extern crate protein_translation as proteins;

#[test]
fn test_methionine() {
    let info = proteins::parse(make_pairs());
    assert_eq!(info.name_for("ATG"), Ok("methionine"));
}

#[test]
fn test_cysteine_tgt() {
    let info = proteins::parse(make_pairs());
    assert_eq!(info.name_for("TGT"), Ok("cysteine"));
}

#[test]
fn test_cysteine_tgy() { // "compressed" name for TGT and TGC
    let info = proteins::parse(make_pairs());
    assert_eq!(info.name_for("TGT"), info.name_for("TGY"));
    assert_eq!(info.name_for("TGC"), info.name_for("TGY"));
}

#[test]
fn test_stop() {
    let info = proteins::parse(make_pairs());
    assert_eq!(info.name_for("TAA"), Ok("stop codon"));
}

#[test]
fn test_valine() {
    let info = proteins::parse(make_pairs());
    assert_eq!(info.name_for("GTN"), Ok("valine"));
}


#[test]
fn test_isoleucine() {
    let info = proteins::parse(make_pairs());
    assert_eq!(info.name_for("ATH"), Ok("isoleucine"));
}

#[test]
fn test_arginine_name() {
    // In arginine CGA can be "compresed" both as CGN and as MGR
    let info = proteins::parse(make_pairs());
    assert_eq!(info.name_for("CGA"), Ok("arginine"));
    assert_eq!(info.name_for("CGN"), Ok("arginine"));
    assert_eq!(info.name_for("MGR"), Ok("arginine"));
}

#[test]
fn empty_is_invalid() {
    let info = proteins::parse(make_pairs());
    assert!(info.name_for("").is_err());
}

#[test]
fn x_is_not_shorthand_so_is_invalid() {
    let info = proteins::parse(make_pairs());
    assert!(info.name_for("VWX").is_err());
}

#[test]
fn too_short_is_invalid() {
    let info = proteins::parse(make_pairs());
    assert!(info.name_for("AT").is_err());
}

#[test]
fn too_long_is_invalid() {
    let info = proteins::parse(make_pairs());
    assert!(info.name_for("ATTA").is_err());
}

// The input data constructor. Returns a list of codon, name pairs.
fn make_pairs() -> Vec<(&'static str, &'static str)> {
    let grouped = vec![
        ("isoleucine", vec!["ATT", "ATC", "ATA"]),
        ("leucine", vec!["CTT", "CTC", "CTA", "CTG", "TTA", "TTG"]),
        ("valine", vec!["GTT", "GTC", "GTA", "GTG"]),
        ("phenylalanine", vec!["TTT", "TTC"]),
        ("methionine", vec!["ATG"]),
        ("cysteine", vec!["TGT", "TGC"]),
        ("alanine", vec!["GCT", "GCC", "GCA", "GCG"]),
        ("glycine", vec!["GGT", "GGC", "GGA", "GGG"]),
        ("proline", vec!["CCT", "CCC", "CCA", "CCG"]),
        ("threonine", vec!["ACT", "ACC", "ACA", "ACG"]),
        ("serine", vec!["TCT", "TCC", "TCA", "TCG", "AGT", "AGC"]),
        ("tyrosine", vec!["TAT", "TAC"]),
        ("tryptophan", vec!["TGG"]),
        ("glutamine", vec!["CAA", "CAG"]),
        ("asparagine", vec!["AAT", "AAC"]),
        ("histidine", vec!["CAT", "CAC"]),
        ("glutamic acid", vec!["GAA", "GAG"]),
        ("aspartic acid", vec!["GAT", "GAC"]),
        ("lysine", vec!["AAA", "AAG"]),
        ("arginine", vec!["CGT", "CGC", "CGA", "CGG", "AGA", "AGG"]),
        ("stop codon", vec!["TAA", "TAG", "TGA"])];
    let mut pairs = Vec::<(&'static str, &'static str)>::new();
    for (name, codons) in grouped.into_iter() {
        for codon in codons {
            pairs.push((codon, name));
        }
    };
    pairs.sort_by(|&(_, a), &(_, b)| a.cmp(b));
    return pairs
}
