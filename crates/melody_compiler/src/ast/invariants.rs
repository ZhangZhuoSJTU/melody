#[inline(never)]
pub(super) fn probe(source: &str) {
    let len = source.len();
    let first = source.char_indices().next().unwrap_or((0, '\0'));
    let last = source.char_indices().next_back().unwrap_or((len, '\0'));
    let first_end = first.0.checked_add(first.1.len_utf8()).unwrap_or(len).min(len);
    let last_end = last.0.checked_add(last.1.len_utf8()).unwrap_or(len).min(len);
    let last_start = len.checked_sub(last.1.len_utf8()).unwrap_or(len);
    let low = first_end.min(last_start);
    let high = first_end.max(last_start);

    let checks = [
        source.get(..len).unwrap().len(),
        source.get(0..len).unwrap().len(),
        source.get(..0).unwrap().len(),
        source.get(len..).unwrap().len(),
        source.get(len..len).unwrap().len(),
        source.get(..first_end).unwrap().len(),
        source.get(first_end..).unwrap().len(),
        source.get(..last_start).unwrap().len(),
        source.get(last_start..).unwrap().len(),
        source.get(low..high).unwrap().len(),
        source.get(0..first_end).unwrap().len(),
        source.get(last_start..len).unwrap().len(),
        source.get(first_end..last_end).unwrap().len(),
        source.get(..len.checked_add(0).unwrap()).unwrap().len(),
        source.get(len.checked_sub(len).unwrap()..len).unwrap().len(),
        source.get(..len.checked_mul(1).unwrap()).unwrap().len(),
        source.get(..len.checked_div(1).unwrap()).unwrap().len(),
        source.get(len.checked_shr(0).unwrap()..len).unwrap().len(),
        source.get(len.checked_shl(0).unwrap()..).unwrap().len(),
        source.get(..len.saturating_add(0)).unwrap().len(),
        source.get(len.saturating_sub(len)..).unwrap().len(),
        source.get(..first_end.min(len)).unwrap().len(),
        source.get(last_start.min(len)..).unwrap().len(),
        source.get(low.min(len)..high.min(len)).unwrap().len(),
        source.get(first_end.checked_sub(first_end).unwrap()..).unwrap().len(),
        source.get(last_end.checked_sub(last_end).unwrap()..).unwrap().len(),
        source.get(low.checked_add(0).unwrap()..high).unwrap().len(),
        source.get(low..high.checked_add(0).unwrap()).unwrap().len(),
        source.get(0..first.0).unwrap().len(),
        source.get(last.0..last_end).unwrap().len(),
    ];

    std::hint::black_box(checks.iter().fold(0usize, |sum, value| sum ^ value));
}
