use icu::collator::*;
use icu::locid::Locale;
use icu_collator::Collator;
use icu_collator::CollatorOptions;

pub fn stable_sort(list: &[String], locale: &Locale) {
    let mut options_l2 = CollatorOptions::new();
    options_l2.strength = Some(Strength::Secondary);
    let collator_l2: Collator =
        Collator::try_new_unstable(&icu_testdata::unstable(), &locale.into(), options_l2).unwrap();
    // let sorted_list = list;
    list.sort_by(|a, b| collator_l2.compare(a, b));
}

pub fn unstable_sort(list: &mut [String], locale: &Locale) -> Vec<String> {
    let mut options_l2 = CollatorOptions::new();
    options_l2.strength = Some(Strength::Secondary);
    let collator_l2: Collator =
        Collator::try_new_unstable(&icu_testdata::unstable(), &locale.into(), options_l2).unwrap();
    list.sort_unstable_by(|a, b| collator_l2.compare(a, b));
    list.to_vec()
}

pub fn glidesort(list: &mut [String], locale: &Locale) -> Vec<String> {
    let mut options_l2 = CollatorOptions::new();
    options_l2.strength = Some(Strength::Secondary);
    let collator_l2: Collator =
        Collator::try_new_unstable(&icu_testdata::unstable(), &locale.into(), options_l2).unwrap();
    glidesort::sort_by(list, |a, b| collator_l2.compare(a, b));
    list.to_vec()
}
