use termion::event::Key;

pub trait NewFromString {
    fn new_from_string(s: &str) -> Key;
}

impl NewFromString for Key {
    fn new_from_string(s: &str) -> Key {
        match s {
            "<Esc>" => Key::Esc,
            "<Null>" => Key::Null,
            "<Backspace>" => Key::Backspace,
            "<Left>" => Key::Left,
            "<Right>" => Key::Right,
            "<Up>" => Key::Up,
            "<Down>" => Key::Down,
            "<Home>" => Key::Home,
            "<End>" => Key::End,
            "<PageUp>" => Key::PageUp,
            "<PageDown>" => Key::PageDown,
            "<BackTab>" => Key::BackTab,
            "<Delete>" => Key::Delete,
            "<Insert>" => Key::Insert,
            "<F1>" => Key::F(1),
            "<F2>" => Key::F(2),
            "<F3>" => Key::F(3),
            "<F4>" => Key::F(4),
            "<F5>" => Key::F(5),
            "<F6>" => Key::F(6),
            "<F7>" => Key::F(7),
            "<F8>" => Key::F(8),
            "<F9>" => Key::F(9),
            "<F10>" => Key::F(10),
            "<F11>" => Key::F(11),
            "<F12>" => Key::F(12),
            "a" => Key::Char('a'),
            "b" => Key::Char('b'),
            "c" => Key::Char('c'),
            "d" => Key::Char('d'),
            "e" => Key::Char('e'),
            "f" => Key::Char('f'),
            "g" => Key::Char('g'),
            "h" => Key::Char('h'),
            "i" => Key::Char('i'),
            "j" => Key::Char('j'),
            "k" => Key::Char('k'),
            "l" => Key::Char('l'),
            "m" => Key::Char('m'),
            "n" => Key::Char('n'),
            "o" => Key::Char('o'),
            "p" => Key::Char('p'),
            "q" => Key::Char('q'),
            "r" => Key::Char('r'),
            "s" => Key::Char('s'),
            "t" => Key::Char('t'),
            "u" => Key::Char('u'),
            "v" => Key::Char('v'),
            "w" => Key::Char('w'),
            "x" => Key::Char('x'),
            "y" => Key::Char('y'),
            "z" => Key::Char('z'),
            "A" => Key::Char('A'),
            "B" => Key::Char('B'),
            "C" => Key::Char('C'),
            "D" => Key::Char('D'),
            "E" => Key::Char('E'),
            "F" => Key::Char('F'),
            "G" => Key::Char('G'),
            "H" => Key::Char('H'),
            "I" => Key::Char('I'),
            "J" => Key::Char('J'),
            "K" => Key::Char('K'),
            "L" => Key::Char('L'),
            "M" => Key::Char('M'),
            "N" => Key::Char('N'),
            "O" => Key::Char('O'),
            "P" => Key::Char('P'),
            "Q" => Key::Char('Q'),
            "R" => Key::Char('R'),
            "S" => Key::Char('S'),
            "T" => Key::Char('T'),
            "U" => Key::Char('U'),
            "V" => Key::Char('V'),
            "W" => Key::Char('W'),
            "X" => Key::Char('X'),
            "Y" => Key::Char('Y'),
            "Z" => Key::Char('Z'),
            "<A-a>" => Key::Alt('a'),
            "<A-b>" => Key::Alt('b'),
            "<A-c>" => Key::Alt('c'),
            "<A-d>" => Key::Alt('d'),
            "<A-e>" => Key::Alt('e'),
            "<A-f>" => Key::Alt('f'),
            "<A-g>" => Key::Alt('g'),
            "<A-h>" => Key::Alt('h'),
            "<A-i>" => Key::Alt('i'),
            "<A-j>" => Key::Alt('j'),
            "<A-k>" => Key::Alt('k'),
            "<A-l>" => Key::Alt('l'),
            "<A-m>" => Key::Alt('m'),
            "<A-n>" => Key::Alt('n'),
            "<A-o>" => Key::Alt('o'),
            "<A-p>" => Key::Alt('p'),
            "<A-q>" => Key::Alt('q'),
            "<A-r>" => Key::Alt('r'),
            "<A-s>" => Key::Alt('s'),
            "<A-t>" => Key::Alt('t'),
            "<A-u>" => Key::Alt('u'),
            "<A-v>" => Key::Alt('v'),
            "<A-w>" => Key::Alt('w'),
            "<A-x>" => Key::Alt('x'),
            "<A-y>" => Key::Alt('y'),
            "<A-z>" => Key::Alt('z'),
            "<A-A>" => Key::Alt('A'),
            "<A-B>" => Key::Alt('B'),
            "<A-C>" => Key::Alt('C'),
            "<A-D>" => Key::Alt('D'),
            "<A-E>" => Key::Alt('E'),
            "<A-F>" => Key::Alt('F'),
            "<A-G>" => Key::Alt('G'),
            "<A-H>" => Key::Alt('H'),
            "<A-I>" => Key::Alt('I'),
            "<A-J>" => Key::Alt('J'),
            "<A-K>" => Key::Alt('K'),
            "<A-L>" => Key::Alt('L'),
            "<A-M>" => Key::Alt('M'),
            "<A-N>" => Key::Alt('N'),
            "<A-O>" => Key::Alt('O'),
            "<A-P>" => Key::Alt('P'),
            "<A-Q>" => Key::Alt('Q'),
            "<A-R>" => Key::Alt('R'),
            "<A-S>" => Key::Alt('S'),
            "<A-T>" => Key::Alt('T'),
            "<A-U>" => Key::Alt('U'),
            "<A-V>" => Key::Alt('V'),
            "<A-W>" => Key::Alt('W'),
            "<A-X>" => Key::Alt('X'),
            "<A-Y>" => Key::Alt('Y'),
            "<A-Z>" => Key::Alt('Z'),
            "<C-a>" => Key::Ctrl('a'),
            "<C-b>" => Key::Ctrl('b'),
            "<C-c>" => Key::Ctrl('c'),
            "<C-d>" => Key::Ctrl('d'),
            "<C-e>" => Key::Ctrl('e'),
            "<C-f>" => Key::Ctrl('f'),
            "<C-g>" => Key::Ctrl('g'),
            "<C-h>" => Key::Ctrl('h'),
            "<C-i>" => Key::Ctrl('i'),
            "<C-j>" => Key::Ctrl('j'),
            "<C-k>" => Key::Ctrl('k'),
            "<C-l>" => Key::Ctrl('l'),
            "<C-m>" => Key::Ctrl('m'),
            "<C-n>" => Key::Ctrl('n'),
            "<C-o>" => Key::Ctrl('o'),
            "<C-p>" => Key::Ctrl('p'),
            "<C-q>" => Key::Ctrl('q'),
            "<C-r>" => Key::Ctrl('r'),
            "<C-s>" => Key::Ctrl('s'),
            "<C-t>" => Key::Ctrl('t'),
            "<C-u>" => Key::Ctrl('u'),
            "<C-v>" => Key::Ctrl('v'),
            "<C-w>" => Key::Ctrl('w'),
            "<C-x>" => Key::Ctrl('x'),
            "<C-y>" => Key::Ctrl('y'),
            "<C-z>" => Key::Ctrl('z'),
            "<C-A>" => Key::Ctrl('A'),
            "<C-B>" => Key::Ctrl('B'),
            "<C-C>" => Key::Ctrl('C'),
            "<C-D>" => Key::Ctrl('D'),
            "<C-E>" => Key::Ctrl('E'),
            "<C-F>" => Key::Ctrl('F'),
            "<C-G>" => Key::Ctrl('G'),
            "<C-H>" => Key::Ctrl('H'),
            "<C-I>" => Key::Ctrl('I'),
            "<C-J>" => Key::Ctrl('J'),
            "<C-K>" => Key::Ctrl('K'),
            "<C-L>" => Key::Ctrl('L'),
            "<C-M>" => Key::Ctrl('M'),
            "<C-N>" => Key::Ctrl('N'),
            "<C-O>" => Key::Ctrl('O'),
            "<C-P>" => Key::Ctrl('P'),
            "<C-Q>" => Key::Ctrl('Q'),
            "<C-R>" => Key::Ctrl('R'),
            "<C-S>" => Key::Ctrl('S'),
            "<C-T>" => Key::Ctrl('T'),
            "<C-U>" => Key::Ctrl('U'),
            "<C-V>" => Key::Ctrl('V'),
            "<C-W>" => Key::Ctrl('W'),
            "<C-X>" => Key::Ctrl('X'),
            "<C-Y>" => Key::Ctrl('Y'),
            "<C-Z>" => Key::Ctrl('Z'),

            "0" => Key::Char('0'),
            "1" => Key::Char('1'),
            "2" => Key::Char('2'),
            "3" => Key::Char('3'),
            "4" => Key::Char('4'),
            "5" => Key::Char('5'),
            "6" => Key::Char('6'),
            "7" => Key::Char('7'),
            "8" => Key::Char('8'),
            "9" => Key::Char('9'),
            "-" => Key::Char('-'),
            "=" => Key::Char('='),
            "`" => Key::Char('`'),
            "[" => Key::Char('['),
            "]" => Key::Char(']'),
            "\\" => Key::Char('\\'),
            ";" => Key::Char(';'),
            "\'" => Key::Char('\''),
            "," => Key::Char(','),
            "." => Key::Char('.'),
            "/" => Key::Char('/'),
            ")" => Key::Char(')'),
            "!" => Key::Char('!'),
            "@" => Key::Char('@'),
            "#" => Key::Char('#'),
            "$" => Key::Char('$'),
            "%" => Key::Char('%'),
            "^" => Key::Char('^'),
            "&" => Key::Char('&'),
            "*" => Key::Char('*'),
            "(" => Key::Char('('),
            "_" => Key::Char('_'),
            "+" => Key::Char('+'),
            "~" => Key::Char('~'),
            "{" => Key::Char('{'),
            "}" => Key::Char('}'),
            "|" => Key::Char('|'),
            ":" => Key::Char(':'),
            "\"" => Key::Char('"'),
            "<" => Key::Char('<'),
            ">" => Key::Char('>'),
            "?" => Key::Char('?'),
            "<C-0>" => Key::Ctrl('0'),
            "<C-1>" => Key::Ctrl('1'),
            "<C-2>" => Key::Ctrl('2'),
            "<C-3>" => Key::Ctrl('3'),
            "<C-4>" => Key::Ctrl('4'),
            "<C-5>" => Key::Ctrl('5'),
            "<C-6>" => Key::Ctrl('6'),
            "<C-7>" => Key::Ctrl('7'),
            "<C-8>" => Key::Ctrl('8'),
            "<C-9>" => Key::Ctrl('9'),
            "<C-->" => Key::Ctrl('-'),
            "<C-=>" => Key::Ctrl('='),
            "<C-`>" => Key::Ctrl('`'),
            "<C-[>" => Key::Ctrl('['),
            "<C-]>" => Key::Ctrl(']'),
            "<C-\\>" => Key::Ctrl('\\'),
            "<C-;>" => Key::Ctrl(';'),
            "<C-\'>" => Key::Ctrl('\''),
            "<C-,>" => Key::Ctrl(','),
            "<C-.>" => Key::Ctrl('.'),
            "<C-/>" => Key::Ctrl('/'),
            "<C-)>" => Key::Ctrl(')'),
            "<C-!>" => Key::Ctrl('!'),
            "<C-@>" => Key::Ctrl('@'),
            "<C-#>" => Key::Ctrl('#'),
            "<C-$>" => Key::Ctrl('$'),
            "<C-%>" => Key::Ctrl('%'),
            "<C-^>" => Key::Ctrl('^'),
            "<C-&>" => Key::Ctrl('&'),
            "<C-*>" => Key::Ctrl('*'),
            "<C-(>" => Key::Ctrl('('),
            "<C-_>" => Key::Ctrl('_'),
            "<C-+>" => Key::Ctrl('+'),
            "<C-~>" => Key::Ctrl('~'),
            "<C-{>" => Key::Ctrl('{'),
            "<C-}>" => Key::Ctrl('}'),
            "<C-|>" => Key::Ctrl('|'),
            "<C-:>" => Key::Ctrl(':'),
            "<C-\">" => Key::Ctrl('"'),
            "<C-<>" => Key::Ctrl('<'),
            "<C->>" => Key::Ctrl('>'),
            "<C-?>" => Key::Ctrl('?'),
            "<A-0>" => Key::Alt('0'),
            "<A-1>" => Key::Alt('1'),
            "<A-2>" => Key::Alt('2'),
            "<A-3>" => Key::Alt('3'),
            "<A-4>" => Key::Alt('4'),
            "<A-5>" => Key::Alt('5'),
            "<A-6>" => Key::Alt('6'),
            "<A-7>" => Key::Alt('7'),
            "<A-8>" => Key::Alt('8'),
            "<A-9>" => Key::Alt('9'),
            "<A-->" => Key::Alt('-'),
            "<A-=>" => Key::Alt('='),
            "<A-`>" => Key::Alt('`'),
            "<A-[>" => Key::Alt('['),
            "<A-]>" => Key::Alt(']'),
            "<A-\\>" => Key::Alt('\\'),
            "<A-;>" => Key::Alt(';'),
            "<A-\'>" => Key::Alt('\''),
            "<A-,>" => Key::Alt(','),
            "<A-.>" => Key::Alt('.'),
            "<A-/>" => Key::Alt('/'),
            "<A-)>" => Key::Alt(')'),
            "<A-!>" => Key::Alt('!'),
            "<A-@>" => Key::Alt('@'),
            "<A-#>" => Key::Alt('#'),
            "<A-$>" => Key::Alt('$'),
            "<A-%>" => Key::Alt('%'),
            "<A-^>" => Key::Alt('^'),
            "<A-&>" => Key::Alt('&'),
            "<A-*>" => Key::Alt('*'),
            "<A-(>" => Key::Alt('('),
            "<A-_>" => Key::Alt('_'),
            "<A-+>" => Key::Alt('+'),
            "<A-~>" => Key::Alt('~'),
            "<A-{>" => Key::Alt('{'),
            "<A-}>" => Key::Alt('}'),
            "<A-|>" => Key::Alt('|'),
            "<A-:>" => Key::Alt(':'),
            "<A-\">" => Key::Alt('"'),
            "<A-<>" => Key::Alt('<'),
            "<A->>" => Key::Alt('>'),
            "<A-?>" => Key::Alt('?'),

            string => todo!("{:?} has no string to key implementation", string),
        }
    }
}

pub trait Display {
    fn display(&self) -> &'static str;
}

impl Display for Key {
    fn display(&self) -> &'static str {
        match self {
            Key::Esc => "<Esc>",
            Key::Null => "<Null>",
            Key::Backspace => "<Backspace>",
            Key::Left => "<Left>",
            Key::Right => "<Right>",
            Key::Up => "<Up>",
            Key::Down => "<Down>",
            Key::Home => "<Home>",
            Key::End => "<End>",
            Key::PageUp => "<PageUp>",
            Key::PageDown => "<PageDown>",
            Key::BackTab => "<BackTab>",
            Key::Delete => "<Delete>",
            Key::Insert => "<Insert>",
            Key::F(1) => "<F1>",
            Key::F(2) => "<F2>",
            Key::F(3) => "<F3>",
            Key::F(4) => "<F4>",
            Key::F(5) => "<F5>",
            Key::F(6) => "<F6>",
            Key::F(7) => "<F7>",
            Key::F(8) => "<F8>",
            Key::F(9) => "<F9>",
            Key::F(10) => "<F10>",
            Key::F(11) => "<F11>",
            Key::F(12) => "<F12>",
            Key::Char('a') => "a",
            Key::Char('b') => "b",
            Key::Char('c') => "c",
            Key::Char('d') => "d",
            Key::Char('e') => "e",
            Key::Char('f') => "f",
            Key::Char('g') => "g",
            Key::Char('h') => "h",
            Key::Char('i') => "i",
            Key::Char('j') => "j",
            Key::Char('k') => "k",
            Key::Char('l') => "l",
            Key::Char('m') => "m",
            Key::Char('n') => "n",
            Key::Char('o') => "o",
            Key::Char('p') => "p",
            Key::Char('q') => "q",
            Key::Char('r') => "r",
            Key::Char('s') => "s",
            Key::Char('t') => "t",
            Key::Char('u') => "u",
            Key::Char('v') => "v",
            Key::Char('w') => "w",
            Key::Char('x') => "x",
            Key::Char('y') => "y",
            Key::Char('z') => "z",
            Key::Char('A') => "A",
            Key::Char('B') => "B",
            Key::Char('C') => "C",
            Key::Char('D') => "D",
            Key::Char('E') => "E",
            Key::Char('F') => "F",
            Key::Char('G') => "G",
            Key::Char('H') => "H",
            Key::Char('I') => "I",
            Key::Char('J') => "J",
            Key::Char('K') => "K",
            Key::Char('L') => "L",
            Key::Char('M') => "M",
            Key::Char('N') => "N",
            Key::Char('O') => "O",
            Key::Char('P') => "P",
            Key::Char('Q') => "Q",
            Key::Char('R') => "R",
            Key::Char('S') => "S",
            Key::Char('T') => "T",
            Key::Char('U') => "U",
            Key::Char('V') => "V",
            Key::Char('W') => "W",
            Key::Char('X') => "X",
            Key::Char('Y') => "Y",
            Key::Char('Z') => "Z",
            Key::Alt('a') => "<A-a>",
            Key::Alt('b') => "<A-b>",
            Key::Alt('c') => "<A-c>",
            Key::Alt('d') => "<A-d>",
            Key::Alt('e') => "<A-e>",
            Key::Alt('f') => "<A-f>",
            Key::Alt('g') => "<A-g>",
            Key::Alt('h') => "<A-h>",
            Key::Alt('i') => "<A-i>",
            Key::Alt('j') => "<A-j>",
            Key::Alt('k') => "<A-k>",
            Key::Alt('l') => "<A-l>",
            Key::Alt('m') => "<A-m>",
            Key::Alt('n') => "<A-n>",
            Key::Alt('o') => "<A-o>",
            Key::Alt('p') => "<A-p>",
            Key::Alt('q') => "<A-q>",
            Key::Alt('r') => "<A-r>",
            Key::Alt('s') => "<A-s>",
            Key::Alt('t') => "<A-t>",
            Key::Alt('u') => "<A-u>",
            Key::Alt('v') => "<A-v>",
            Key::Alt('w') => "<A-w>",
            Key::Alt('x') => "<A-x>",
            Key::Alt('y') => "<A-y>",
            Key::Alt('z') => "<A-z>",
            Key::Alt('A') => "<A-A>",
            Key::Alt('B') => "<A-B>",
            Key::Alt('C') => "<A-C>",
            Key::Alt('D') => "<A-D>",
            Key::Alt('E') => "<A-E>",
            Key::Alt('F') => "<A-F>",
            Key::Alt('G') => "<A-G>",
            Key::Alt('H') => "<A-H>",
            Key::Alt('I') => "<A-I>",
            Key::Alt('J') => "<A-J>",
            Key::Alt('K') => "<A-K>",
            Key::Alt('L') => "<A-L>",
            Key::Alt('M') => "<A-M>",
            Key::Alt('N') => "<A-N>",
            Key::Alt('O') => "<A-O>",
            Key::Alt('P') => "<A-P>",
            Key::Alt('Q') => "<A-Q>",
            Key::Alt('R') => "<A-R>",
            Key::Alt('S') => "<A-S>",
            Key::Alt('T') => "<A-T>",
            Key::Alt('U') => "<A-U>",
            Key::Alt('V') => "<A-V>",
            Key::Alt('W') => "<A-W>",
            Key::Alt('X') => "<A-X>",
            Key::Alt('Y') => "<A-Y>",
            Key::Alt('Z') => "<A-Z>",
            Key::Ctrl('a') => "<C-a>",
            Key::Ctrl('b') => "<C-b>",
            Key::Ctrl('c') => "<C-c>",
            Key::Ctrl('d') => "<C-d>",
            Key::Ctrl('e') => "<C-e>",
            Key::Ctrl('f') => "<C-f>",
            Key::Ctrl('g') => "<C-g>",
            Key::Ctrl('h') => "<C-h>",
            Key::Ctrl('i') => "<C-i>",
            Key::Ctrl('j') => "<C-j>",
            Key::Ctrl('k') => "<C-k>",
            Key::Ctrl('l') => "<C-l>",
            Key::Ctrl('m') => "<C-m>",
            Key::Ctrl('n') => "<C-n>",
            Key::Ctrl('o') => "<C-o>",
            Key::Ctrl('p') => "<C-p>",
            Key::Ctrl('q') => "<C-q>",
            Key::Ctrl('r') => "<C-r>",
            Key::Ctrl('s') => "<C-s>",
            Key::Ctrl('t') => "<C-t>",
            Key::Ctrl('u') => "<C-u>",
            Key::Ctrl('v') => "<C-v>",
            Key::Ctrl('w') => "<C-w>",
            Key::Ctrl('x') => "<C-x>",
            Key::Ctrl('y') => "<C-y>",
            Key::Ctrl('z') => "<C-z>",
            Key::Ctrl('A') => "<C-A>",
            Key::Ctrl('B') => "<C-B>",
            Key::Ctrl('C') => "<C-C>",
            Key::Ctrl('D') => "<C-D>",
            Key::Ctrl('E') => "<C-E>",
            Key::Ctrl('F') => "<C-F>",
            Key::Ctrl('G') => "<C-G>",
            Key::Ctrl('H') => "<C-H>",
            Key::Ctrl('I') => "<C-I>",
            Key::Ctrl('J') => "<C-J>",
            Key::Ctrl('K') => "<C-K>",
            Key::Ctrl('L') => "<C-L>",
            Key::Ctrl('M') => "<C-M>",
            Key::Ctrl('N') => "<C-N>",
            Key::Ctrl('O') => "<C-O>",
            Key::Ctrl('P') => "<C-P>",
            Key::Ctrl('Q') => "<C-Q>",
            Key::Ctrl('R') => "<C-R>",
            Key::Ctrl('S') => "<C-S>",
            Key::Ctrl('T') => "<C-T>",
            Key::Ctrl('U') => "<C-U>",
            Key::Ctrl('V') => "<C-V>",
            Key::Ctrl('W') => "<C-W>",
            Key::Ctrl('X') => "<C-X>",
            Key::Ctrl('Y') => "<C-Y>",
            Key::Ctrl('Z') => "<C-Z>",

            Key::Char('0') => "0",
            Key::Char('1') => "1",
            Key::Char('2') => "2",
            Key::Char('3') => "3",
            Key::Char('4') => "4",
            Key::Char('5') => "5",
            Key::Char('6') => "6",
            Key::Char('7') => "7",
            Key::Char('8') => "8",
            Key::Char('9') => "9",
            Key::Char('-') => "-",
            Key::Char('=') => "=",
            Key::Char('`') => "`",
            Key::Char('[') => "[",
            Key::Char(']') => "]",
            Key::Char('\\') => "\\",
            Key::Char(';') => ";",
            Key::Char('\'') => "\'",
            Key::Char(',') => ",",
            Key::Char('.') => ".",
            Key::Char('/') => "/",
            Key::Char(')') => ")",
            Key::Char('!') => "!",
            Key::Char('@') => "@",
            Key::Char('#') => "#",
            Key::Char('$') => "$",
            Key::Char('%') => "%",
            Key::Char('^') => "^",
            Key::Char('&') => "&",
            Key::Char('*') => "*",
            Key::Char('(') => "(",
            Key::Char('_') => "_",
            Key::Char('+') => "+",
            Key::Char('~') => "~",
            Key::Char('{') => "{",
            Key::Char('}') => "}",
            Key::Char('|') => "|",
            Key::Char(':') => ":",
            Key::Char('"') => "\"",
            Key::Char('<') => "<",
            Key::Char('>') => ">",
            Key::Char('?') => "?",
            Key::Ctrl('0') => "<C-0>",
            Key::Ctrl('1') => "<C-1>",
            Key::Ctrl('2') => "<C-2>",
            Key::Ctrl('3') => "<C-3>",
            Key::Ctrl('4') => "<C-4>",
            Key::Ctrl('5') => "<C-5>",
            Key::Ctrl('6') => "<C-6>",
            Key::Ctrl('7') => "<C-7>",
            Key::Ctrl('8') => "<C-8>",
            Key::Ctrl('9') => "<C-9>",
            Key::Ctrl('-') => "<C-->",
            Key::Ctrl('=') => "<C-=>",
            Key::Ctrl('`') => "<C-`>",
            Key::Ctrl('[') => "<C-[>",
            Key::Ctrl(']') => "<C-]>",
            Key::Ctrl('\\') => "<C-\\>",
            Key::Ctrl(';') => "<C-;>",
            Key::Ctrl('\'') => "<C-\'>",
            Key::Ctrl(',') => "<C-,>",
            Key::Ctrl('.') => "<C-.>",
            Key::Ctrl('/') => "<C-/>",
            Key::Ctrl(')') => "<C-)>",
            Key::Ctrl('!') => "<C-!>",
            Key::Ctrl('@') => "<C-@>",
            Key::Ctrl('#') => "<C-#>",
            Key::Ctrl('$') => "<C-$>",
            Key::Ctrl('%') => "<C-%>",
            Key::Ctrl('^') => "<C-^>",
            Key::Ctrl('&') => "<C-&>",
            Key::Ctrl('*') => "<C-*>",
            Key::Ctrl('(') => "<C-(>",
            Key::Ctrl('_') => "<C-_>",
            Key::Ctrl('+') => "<C-+>",
            Key::Ctrl('~') => "<C-~>",
            Key::Ctrl('{') => "<C-{>",
            Key::Ctrl('}') => "<C-}>",
            Key::Ctrl('|') => "<C-|>",
            Key::Ctrl(':') => "<C-:>",
            Key::Ctrl('"') => "<C-\">",
            Key::Ctrl('<') => "<C-<>",
            Key::Ctrl('>') => "<C->>",
            Key::Ctrl('?') => "<C-?>",
            Key::Alt('0') => "<A-0>",
            Key::Alt('1') => "<A-1>",
            Key::Alt('2') => "<A-2>",
            Key::Alt('3') => "<A-3>",
            Key::Alt('4') => "<A-4>",
            Key::Alt('5') => "<A-5>",
            Key::Alt('6') => "<A-6>",
            Key::Alt('7') => "<A-7>",
            Key::Alt('8') => "<A-8>",
            Key::Alt('9') => "<A-9>",
            Key::Alt('-') => "<A-->",
            Key::Alt('=') => "<A-=>",
            Key::Alt('`') => "<A-`>",
            Key::Alt('[') => "<A-[>",
            Key::Alt(']') => "<A-]>",
            Key::Alt('\\') => "<A-\\>",
            Key::Alt(';') => "<A-;>",
            Key::Alt('\'') => "<A-\'>",
            Key::Alt(',') => "<A-,>",
            Key::Alt('.') => "<A-.>",
            Key::Alt('/') => "<A-/>",
            Key::Alt(')') => "<A-)>",
            Key::Alt('!') => "<A-!>",
            Key::Alt('@') => "<A-@>",
            Key::Alt('#') => "<A-#>",
            Key::Alt('$') => "<A-$>",
            Key::Alt('%') => "<A-%>",
            Key::Alt('^') => "<A-^>",
            Key::Alt('&') => "<A-&>",
            Key::Alt('*') => "<A-*>",
            Key::Alt('(') => "<A-(>",
            Key::Alt('_') => "<A-_>",
            Key::Alt('+') => "<A-+>",
            Key::Alt('~') => "<A-~>",
            Key::Alt('{') => "<A-{>",
            Key::Alt('}') => "<A-}>",
            Key::Alt('|') => "<A-|>",
            Key::Alt(':') => "<A-:>",
            Key::Alt('"') => "<A-\">",
            Key::Alt('<') => "<A-<>",
            Key::Alt('>') => "<A->>",
            Key::Alt('?') => "<A-?>",

            key => todo!("{:?} has no display implementation", key),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::keys::NewFromString;
    use termion::event::Key;

    #[test]
    fn esc_from_string() {
        let key_1 = Key::new_from_string("<ESC>");
        let key_2 = Key::new_from_string("<Esc>");
        let key_3 = Key::new_from_string("<esc>");

        assert_eq!(key_1, Key::Esc);
        assert_eq!(key_2, Key::Esc);
        assert_eq!(key_3, Key::Esc);
    }
}
