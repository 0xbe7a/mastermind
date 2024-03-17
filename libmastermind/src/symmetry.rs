use super::positions::StandardCollection;

#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Debug)]
pub enum SignatureSymbol {
    Color(u8),
    Zero,
    Free(u8),
}

fn symetry_signature(
    p: &[u8],
    unknown_colors: [bool; 8],
    zero_colors: [bool; 8],
) -> [SignatureSymbol; 8] {
    let mut s: [SignatureSymbol; 8] = [SignatureSymbol::Zero; 8];
    let mut free_index = 1;
    let mut indexes = [0; 8];
    for (i, pin) in p.iter().enumerate() {
        if unknown_colors[*pin as usize] {
            if indexes[*pin as usize] == 0 {
                indexes[*pin as usize] = free_index;
                free_index += 1;
            }
            s[i] = SignatureSymbol::Free(indexes[*pin as usize]);
        } else if !zero_colors[*pin as usize] {
            s[i] = SignatureSymbol::Color(*pin);
        }
    }
    s
}

impl StandardCollection {
    pub fn prune_symmetrys(&self, previous_guesses: &[Vec<u8>], guesses: &Self) -> Self {
        let eliminated_colors = self.into_iter().fold([true; 8], |mut x, pins| {
            for c in pins {
                x[*c as usize] = false;
            }
            x
        });

        let untested_colors = previous_guesses.iter().fold([true; 8], |mut x, pins| {
            for c in pins.iter() {
                x[*c as usize] = false;
            }
            x
        });

        let mut results: Vec<([SignatureSymbol; 8], &[u8])> = guesses
            .into_iter()
            .map(|pins| {
                (
                    symetry_signature(pins, untested_colors, eliminated_colors),
                    pins,
                )
            })
            .collect();

        results.sort_unstable_by(|(s1, _), (s2, _)| s1.cmp(s2));
        results.dedup_by(|(s1, _), (s2, _)| s1 == s2);

        //Restore order by pin
        results.sort_unstable_by(|(_, p1), (_, p2)| p1.cmp(p2));

        let (pin_count, color_count) = self.get_size();
        let mut pruned_collection = Self::new(self.len(), pin_count, color_count);

        for (_, pins) in results {
            pruned_collection.push(pins);
        }

        pruned_collection


    }
}
