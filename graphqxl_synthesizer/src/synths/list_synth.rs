use crate::synths::synth::Synth;
use crate::synths::SynthContext;
use crate::utils::is_last_iter;

enum ListSynthSep {
    Inline(String),
    InlineOrMultiline(String, String),
    Multiline(String),
}

pub(crate) struct ListSynth<T: Synth> {
    pub(crate) items: Vec<T>,
    pub(crate) sep: ListSynthSep,
    pub(crate) wrapper: (String, String),
}

impl<T: Synth> Synth for ListSynth<T> {
    fn synth(&self, context: &SynthContext) -> String {
        let mut result = self.wrapper.0.clone();
        for (is_last, item) in is_last_iter(self.items.iter()) {
            match &self.sep {
                ListSynthSep::Inline(sep) => {
                    result += &item.synth(context);
                    if !is_last {
                        result += sep
                    }
                }
                ListSynthSep::Multiline(sep) => {
                    result += "\n";
                    result += &" ".repeat((context.indent_lvl + 1) * context.indent_spaces);
                    result += &item.synth(&context.plus_one_indent_lvl());
                    if !is_last {
                        result += sep
                    }
                }
                ListSynthSep::InlineOrMultiline(inline_sep, multiline_sep) => {
                    if context.multiline {
                        result += "\n";
                        result += &" ".repeat((context.indent_lvl + 1) * context.indent_spaces);
                        result += &item.synth(&context.plus_one_indent_lvl());
                        if !is_last {
                            result += multiline_sep
                        }
                    } else {
                        result += &item.synth(context);
                        if !is_last {
                            result += inline_sep
                        }
                    }
                }
            }
        }
        match &self.sep {
            ListSynthSep::Inline(_) => {
                // do nothing
            }
            ListSynthSep::Multiline(_) => {
                result += "\n";
                result += &" ".repeat(context.indent_lvl * context.indent_spaces);
            }
            ListSynthSep::InlineOrMultiline(_, _) => {
                if context.multiline && !self.wrapper.1.is_empty() {
                    result += "\n";
                    result += &" ".repeat(context.indent_lvl * context.indent_spaces);
                } else {
                    // do nothing
                }
            }
        }
        result + &self.wrapper.1
    }
}

impl<T: Synth> ListSynth<T> {
    pub(crate) fn inline(tuple: (&str, Vec<T>, &str)) -> Self {
        Self {
            items: tuple.1,
            sep: ListSynthSep::Inline(" ".to_string()),
            wrapper: (tuple.0.to_string(), tuple.2.to_string()),
        }
    }

    pub(crate) fn inline_suffixed(tuple: (&str, Vec<T>, &str, &str)) -> Self {
        Self {
            items: tuple.1,
            sep: ListSynthSep::Inline(tuple.2.to_string()),
            wrapper: (tuple.0.to_string(), tuple.3.to_string()),
        }
    }

    pub(crate) fn multiline(tuple: (&str, Vec<T>, &str)) -> Self {
        Self {
            items: tuple.1,
            sep: ListSynthSep::Multiline("".to_string()),
            wrapper: (tuple.0.to_string(), tuple.2.to_string()),
        }
    }

    pub(crate) fn multiline_suffixed(tuple: (&str, Vec<T>, &str, &str)) -> Self {
        Self {
            items: tuple.1,
            sep: ListSynthSep::Multiline(tuple.2.to_string()),
            wrapper: (tuple.0.to_string(), tuple.3.to_string()),
        }
    }

    pub(crate) fn inline_or_multiline(tuple: (&str, Vec<T>, &str)) -> Self {
        Self {
            items: tuple.1,
            sep: ListSynthSep::InlineOrMultiline("".to_string(), "".to_string()),
            wrapper: (tuple.0.to_string(), tuple.2.to_string()),
        }
    }

    pub(crate) fn inline_or_multiline_suffixed(tuple: (&str, Vec<T>, (&str, &str), &str)) -> Self {
        Self {
            items: tuple.1,
            sep: ListSynthSep::InlineOrMultiline(tuple.2 .0.to_string(), tuple.2 .1.to_string()),
            wrapper: (tuple.0.to_string(), tuple.3.to_string()),
        }
    }
}
