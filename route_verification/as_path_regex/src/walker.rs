use super::*;

use Remaining::*;

pub struct Walker<'a> {
    itp: &'a Interpreter,
    init_state: &'a HirKind,
    rems: Vec<Remaining<'a>>,
    has_err: bool,
}

impl<'a> Walker<'a> {
    pub(crate) fn new(interpreter: &'a Interpreter, remaining: &'a HirKind) -> Self {
        Self {
            itp: interpreter,
            init_state: remaining,
            rems: vec![Ir(remaining)],
            has_err: false,
        }
    }

    pub fn reset(&mut self) {
        self.rems.clear();
        self.rems.push(Ir(self.init_state))
    }

    fn err<O>(&mut self, problem: InterpreteProblem) -> Option<Result<O, InterpreteProblem>> {
        self.has_err = true;
        Some(Err(problem))
    }
}

type InnerNext<'a> = Result<Event<'a>, InterpreteProblem>;
type Next<'a> = Option<InnerNext<'a>>;

impl<'a> Iterator for Walker<'a> {
    type Item = InnerNext<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        next(self)
    }
}

fn next<'a>(walker: &mut Walker<'a>) -> Next<'a> {
    if walker.has_err {
        return Some(Err(InterpreteProblem::HadErr));
    }
    let rem = walker.rems.pop()?;
    handle_rem(walker, rem)
}

fn handle_rem<'a>(walker: &mut Walker<'a>, rem: Remaining<'a>) -> Next<'a> {
    match rem {
        Ir(hir) => match hir {
            HirKind::Empty => walker.err(InterpreteProblem::Empty),
            HirKind::Literal(Literal(literal)) => handle_ir_literal(walker, literal),
            HirKind::Class(class) => handle_class(walker, class),
            HirKind::Look(look) => Some(handle_look(*look)),
            HirKind::Repetition(repeat) => Some(handle_repeat(walker, repeat)),
            HirKind::Capture(_) => unreachable!("We should not have capture groups."),
            HirKind::Concat(concat) => handle_concat(walker, concat),
            HirKind::Alternation(ors) => handle_ir_ors(walker, ors),
        },
        Lit(literal, index) => {
            let next_char = match literal[index..].chars().next() {
                Some(c) => c,
                None => return next(walker),
            };
            Some(handle_literal(walker, literal, index, next_char))
        }
        Ranges(ranges) => handle_ranges(walker, ranges),
        Range(start, end) => {
            if start > end {
                next(walker)
            } else {
                Some(handle_range(walker, start, end))
            }
        }
        Ors(ors) => handle_ors(walker, ors),
    }
}

fn handle_ir_literal<'a>(walker: &mut Walker<'a>, literal: &[u8]) -> Next<'a> {
    let decoded = match String::from_utf8(literal.to_vec()) {
        Ok(d) => d,
        Err(_) => return walker.err(InterpreteProblem::InvalidRegex),
    };
    walker.rems.push(Lit(decoded, 0));
    next(walker)
}

fn handle_class<'a>(walker: &mut Walker<'a>, class: &'a Class) -> Next<'a> {
    let ranges = match class {
        Class::Unicode(c) => c.ranges(),
        Class::Bytes(_) => unreachable!("This `class` cannot be `Bytes`."),
    };
    walker.rems.push(Ranges(ranges));
    next(walker)
}

fn handle_look<'a>(look: Look) -> InnerNext<'a> {
    match look {
        Look::Start | Look::StartLF | Look::StartCRLF => Ok(Event::Start),
        Look::End | Look::EndLF | Look::EndCRLF => Ok(Event::End),
        _ => Err(InterpreteProblem::InvalidRegex),
    }
}

fn handle_repeat<'a>(
    walker: &mut Walker<'a>,
    Repetition {
        min,
        max,
        greedy,
        sub,
    }: &'a Repetition,
) -> InnerNext<'a> {
    Ok(Event::Repeat {
        min: *min,
        max: *max,
        greedy: *greedy,
        walker: Walker::new(walker.itp, sub.kind()),
    })
}

fn handle_concat<'a>(walker: &mut Walker<'a>, concat: &'a Vec<Hir>) -> Next<'a> {
    walker.rems.reserve(concat.len());
    for hir in concat.iter().rev() {
        walker.rems.push(Ir(hir.kind()));
    }
    next(walker)
}

fn handle_ir_ors<'a>(walker: &mut Walker<'a>, ors: &'a [Hir]) -> Next<'a> {
    walker.rems.push(Ors(ors));
    walker.next()
}

fn handle_ors<'a>(walker: &mut Walker<'a>, ors: &'a [Hir]) -> Next<'a> {
    match ors.first() {
        Some(ir) => {
            let walker = Walker::new(walker.itp, ir.kind());
            Some(Ok(Event::Or(walker)))
        }
        None => walker.next(),
    }
}

fn handle_literal<'a>(
    walker: &mut Walker<'a>,
    literal: String,
    index: usize,
    next_char: char,
) -> InnerNext<'a> {
    let rem = Lit(literal, index + next_char.len_utf8());
    walker.rems.push(rem);
    let as_or_set = walker.itp.get_char(next_char)?;
    Ok(Event::Literal(as_or_set))
}

fn handle_ranges<'a>(walker: &mut Walker<'a>, ranges: &'a [ClassUnicodeRange]) -> Next<'a> {
    if let Some(range) = ranges.first() {
        walker.rems.push(Ranges(&ranges[1..]));
        walker.rems.push(Range(range.start(), range.end()))
    }
    next(walker)
}

fn handle_range<'a>(walker: &mut Walker<'a>, start: char, end: char) -> InnerNext<'a> {
    let as_or_set = walker.itp.get_char(start)?;
    let start = start as u32 + 1;
    // SAFETY: `start` ≤ `end` that is small ⇒ `start` + 1 is small enough.
    let start = unsafe { char::from_u32_unchecked(start) };
    walker.rems.push(Range(start, end));
    Ok(Event::Permit(as_or_set))
}

#[derive(Clone)]
enum Remaining<'a> {
    Ir(&'a HirKind),
    Lit(String, usize),
    Ranges(&'a [ClassUnicodeRange]),
    Range(char, char),
    Ors(&'a [Hir]),
}
