use std;

const SYMRANK_NEXT_ARRAY: [u16; super::SYMRANK_NUM_SYMBOLS] =
    include!(concat!(env!("OUT_DIR"), "/", "SYMRANK_NEXT_ARRAY.txt"));

#[derive(Clone, Copy)]
pub struct SymRankCoder {
    value_array: [u16; super::SYMRANK_NUM_SYMBOLS],
    index_array: [u16; super::SYMRANK_NUM_SYMBOLS],
}

impl SymRankCoder {
    pub fn new() -> SymRankCoder {
        return SymRankCoder {
            value_array: [0; super::SYMRANK_NUM_SYMBOLS],
            index_array: [0; super::SYMRANK_NUM_SYMBOLS],
        };
    }

    pub fn init(&mut self, value_array: &[u16]) {
        for i in 0..super::SYMRANK_NUM_SYMBOLS {
            self.value_array[i] = value_array[i];
            self.index_array[self.value_array[i] as usize] = i as u16;
        }
    }

    pub unsafe fn encode(&mut self, v: u16, vunlikely: u16) -> u16 {
        let self_index_array = &mut unchecked_index::unchecked_index(&mut self.index_array);
        let i = self_index_array[v as usize];
        let iunlikely = self_index_array[vunlikely as usize];

        self.update(v, i);
        return match i.cmp(&iunlikely) {
            std::cmp::Ordering::Less    => i,
            std::cmp::Ordering::Greater => i - 1,
            std::cmp::Ordering::Equal   => super::SYMRANK_NUM_SYMBOLS as u16 - 1,
        };
    }

    pub unsafe fn decode(&mut self, i: u16, vunlikely: u16) -> u16 {
        let self_index_array = &mut unchecked_index::unchecked_index(&mut self.index_array);
        let self_value_array = &mut unchecked_index::unchecked_index(&mut self.value_array);

        let iunlikely = self_index_array[vunlikely as usize];
        let i = match i {
            _ if i < iunlikely => i,
            _ if i < super::SYMRANK_NUM_SYMBOLS as u16 - 1 => i + 1,
            _ => iunlikely,
        };
        let v = self_value_array[i as usize];

        self.update(v, i);
        return v;
    }

    unsafe fn update(&mut self, v: u16, i: u16) {
        let symrank_next_array = &unchecked_index::unchecked_index(SYMRANK_NEXT_ARRAY);
        let self_index_array = &mut unchecked_index::unchecked_index(&mut self.index_array);
        let self_value_array = &mut unchecked_index::unchecked_index(&mut self.value_array);

        if i < 32 {
            let ni1 = symrank_next_array[i as usize];
            let nv1 = self.value_array[ni1 as usize];
            std::ptr::swap(&mut self.index_array[v as usize], &mut self.index_array[nv1 as usize]);
            std::ptr::swap(&mut self.value_array[i as usize], &mut self.value_array[ni1 as usize]);

        } else {
            let ni1 = symrank_next_array[i as usize];
            let ni2 = (i + ni1) / 2;

            let nv2 = self_value_array[ni2 as usize];
            std::ptr::swap(&mut self_index_array[v as usize], &mut self_index_array[nv2 as usize]);
            std::ptr::swap(&mut self_value_array[i as usize], &mut self_value_array[ni2 as usize]);

            let nv1 = self_value_array[ni1 as usize];
            std::ptr::swap(&mut self_index_array[v   as usize], &mut self_index_array[nv1 as usize]);
            std::ptr::swap(&mut self_value_array[ni2 as usize], &mut self_value_array[ni1 as usize]);
        }
    }
}
