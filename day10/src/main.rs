use anyhow::Result;

struct Checker {
    stack: Vec<char>
}

impl Checker {
    fn new() -> Checker {
        Checker {
            stack: Vec::new()
        }
    }

    fn check(&mut self, s: &str) -> Result<(), (char, usize)> {
        let mut pos: usize = 0;
        for ch in s.chars() {
            match ch {
                '(' | '[' | '{' | '<' => self.stack.push(ch),
                ')' | ']' | '}' | '>' => if Checker::closer(self.stack[self.stack.len()-1]) == ch {
                    self.stack.pop();
                } else {
                    return Err((ch, pos))
                },
                _ => panic!("Invalid char {}", ch),
            }
            pos += 1;
        }

        Ok(())
    }

    fn complete(&mut self, s: &str) -> Vec<char> {
        for (pos, ch) in s.chars().enumerate() {
            match ch {
                '(' | '[' | '{' | '<' => self.stack.push(ch),
                ')' | ']' | '}' | '>' => if Checker::closer(self.stack[self.stack.len()-1]) == ch {
                    self.stack.pop();
                } else {
                    panic!("Check failed at {} at {}", ch, pos)
                },
                _ => panic!("Invalid char {}", ch),
            }
        }

        let mut ret = Vec::new();
        for ch in self.stack.iter().rev() {
            ret.push(Checker::closer(*ch));
        }
        ret
    }

    fn closer(ch: char) -> char {
        match ch {
            '(' => ')',
            '{' => '}',
            '[' => ']',
            '<' => '>',
            _ => panic!("Bad stack state {}", ch),
        }
    }
}

fn score(ch: char) -> u64 {
    match ch {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("Can't score {}", ch)
    }
}

fn complete_score(ch: char) -> u64 {
    match ch {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => panic!("Can't score {}", ch)
    }
}

fn main() -> Result<()> {
    let input: Vec<String> = INPUT.lines().map(|l| l.to_string()).collect();

    let mut incompletes = Vec::new();
    let mut check_score_total = 0;
    for line in input {
        let mut checker = Checker::new();
        if let Err((ch, _pos)) = checker.check(&line) {
            check_score_total += score(ch);
        } else {
            incompletes.push(line);
        }
    }

    println!("Part1: {}", check_score_total);

    let mut scores = Vec::new();
    for line in incompletes {
        let mut complete_score_total = 0;
        let mut checker = Checker::new();
        let completion = checker.complete(&line);
        for ch in completion {
            complete_score_total = complete_score_total * 5 + complete_score(ch);
        }
        scores.push(complete_score_total);
    }
    scores.sort();

    println!("Part2: {}", scores[scores.len()/2]);

    Ok(())
}

const INPUT: &str = r#"[{[[{(((<[{{((()<>)(<><>))[{{}{}}((){})]}[<<()[]>[{}()]>[{<>{}}<()()>]]}{{[[{}<>]({}{})]<[{}
<[({(([<[[<([[(){}]<()()>])>[[(({})[<>()])]]]<[(({()[]})[{<>{}>(<>{})])<({<>()}[<>{}])(({}<>){()<>
([{[([(((<<<{({}())<()[]>}<[[]()]{<>{}})>[{[[][]]{<><>}}]>>)[[[{{({}<>)([]<>)}}({(()[])<()[]>})](<[[()()]<{}
(<([<<({<[[[([[]{})<()[]>)(<[]()>[{}<>])]<[{[]{}}<()[]>]<[<><>]>>]<{[({})[<>{}]]<[()[]](()<>)>}{((()
{<({{<{<[<({([()<>]{<>[]})<<{}<>>>}((([]{})((){}))[[(){}](<>[])]))>][[<(<{()()>{{}[]}><{{}()}(()<>)>)
{<([<<[<(<{<<(()[])[{}[]]>>(<{()[]}>({<>()}<[]()>))}>)>[{{{({<[]><(){}>]({{}[]}<{}<>>))<{<(){}><[]{}>}({<>{}}
{[[([{<<{{(({{<>{}]<{}{}>}[<{}<>><()()>]))({({{}[]}<<>[]>)(<<>>{[][]})})}<[(({[]{}}[{}{}])<(<>())<(){
[<(<[{[<{{[<[{{}}([][])]{({}{})(<>{}]}>{<(<>[])({}())>({[]<>}{[]()})}](<{[{}]<()[]>}<([][]){{}()}>>)}<<(<[()(
<([[<{<{<[{[[{()}<[]>]]<(<[]()>)>}([([{}[]]<[]<>>)(<[][]><()<>))]{<<()[]>((){})>([{}<>]([]{}))})]{
(<({[<((([<[{([]())<<><>>)](<{[]}[{}()]>[[{}<>]((){})])>{{{{{}<>}([][])}}}]{[(((<><>)<()()>)[([]<>
<(([<(<{({{{<[()[]]<[]()>)[(<>)<<>>]}[[<(){}>[{}<>]]<[<>()]([]<>)>]}[<[{{}<>}([]{})]{[{}()]}>]})}>(({(
<<(([[{{<[<[([()[]]<()[]>)<([][]){{}<>}>]<{[()[]]({}[])}<(<>[])>>>({<<[]()>{{}})}([<{}[]>[[]()]]))
{{{<<[<[<<[<{(()[])[[]{}]}[[[]()]{{}<>}]>[{(<>)[<>()]}]]>(<{[[()<>]<{}[]>]({{}}[(){}])}{<<[]>(
<([<<[{<[[{([([][])[{}()]][(<><>)[{}<>]])[(({}{}){{}<>})]}((({[][]}<()<>>)(({}{})[()<>])))][(<(({}<
{[<{[([[{[{[<{()<>}<<>[]>>{{<>()}[<><>]}]}]([{([[][]](()()))<{[]<>}[()<>]>}{<<<>{}>{[]<>}>}))}([(<(((){}){(
({<{[{[[{({<<(()<>){<>()}>(<()>{{}()})>({[[]()][{}]}{{<>{}]{{}[]}})})}]<{{[{[<<>[]>][(<>())<<>()>]}<{<{
([(([[[{<({{{[()()]{<>{}}}}<[[[]<>][{}{}]]({()()}{()()})>}({[[(){}][[]{}]]{({}[])[{}()]}}{{[<><>]}
((<[<{(({(<<[{()()>]<(<>)<[]<>>>><{<{}[]>([]())}{{[]()}<{}<>>}>>)}))}[{(<({{{([][])[(){}]}}<<<{}<>>{{}[]}>(<{
{<[[([<([<{([([]{}){{}<>}]([[]{}]({}())))[[<{}[]>{{}<>)]{({}[])[[]]}]}(([[[]{}][<>[]]](({}[]){{}{}}))
[<<[<[({<{([<<<>{}>(()[])><({}[])[{}<>]>]<[<[][]>{()[]}}<[{}()]>>)<<<(()[])>>>}{<{<<{}()>[[]()]>{[<>(
<(<(<[({{<<[[{()<>}[{}<>]][[[]()](()<>)]]{[<()>([]<>)]<[{}{}]<{}{}>>}>([[(()()){(){}}]]<[<[][]>{[]
<[{[{{{{<({[[({}{})][<{}{}>{[]{}}]]{[[()()]{<>[]}]<[<><>]<<><>>>}})>{({(<<()()>(<>{})>{(()[
<[(([(<<(<{<(<{}<>>{{}{}})[[[][]]([]())]>{{<()>[<>{}]}({<><>}({}()))}}>[([{(<><>)[()()]}[[{}[
<((<[{([{[([({{}<>}<[][]>)[<[]<>>(<>{})]])(({[{}[]][<><>]}[{()[]}<{}{}>]))]<<{(<[][]>[<>]][{()(
((<[<<({{((<[(<>())[[]{}]]{{[][]}<<>>}>({<[]()>[[]{}]}[({}[])(()())])){({(<>[])[{}()]}[[{}{}>]){{[()<>]{
{<<{{(({(<({({{}{}}<()<>>)}([[()[]]]))>)})[{([<<<<[]<>>>({()<>})>[[[{}<>]][(<><>)(<>[])]]>])}[(<<<{(
{<[<{[<(([{[<<{}()>([]())>[{<>}{{}[]}]]}][{<[([]{})<()<>>]{{[]()>[{}[]]}>[<<<>()>[<>()]>{([]<>){[]()}}]}[
<{{<{[<<[<<<(<{}[]><()[]>)<[()()]{()[]}>>{<<<>{}>{<>[]}>[<()()>{[]{}}]}><<{{[]{}}{{}{}}}>>>[{{{([]{})<{}[]>}}
(<{(([[{[[[(<[{}()]<{}[]>>{<<>>({}{})})]]]{{{{(<[]>(())){<()[]>({}[])}}[[<(){}>[[]<>]]([{}{}]<[]{}>)]}}[<<{<[
{{((({{[((((<<<>()><[]<>>>[[[]<>]<()()>])({(<>{})}[[{}<>]<{}[]>]))[<(([]())[{}<>])[<[]{}>({}<>)]>])[<[<
{{((<([(([([([[]<>][{}<>])(([]()))])]<(((<[][]>{[]()})([{}{}]{<>()})))>))])[([{{[[[<[]{}>[<><>]]<[[]()]>]<(
<(<[[((<[[{(<{[]{}}><[{}{}][{}[]]>)[<{[]()}[[]{})><[()<>]<{}()>>]}<[<{{}[]}{<>()}>{{<><>}{{}{}}
{{[[<(<[<(<{[{[]}[{}<>]][((){})[()()]]}<([<>{}]{[]{}}){({}{})[<>]}>>{{{{<>[]}(())}{<{}{}><(){}>}}}
(<<(([<[{<<[<(()[])>[{{}}([]<>)]][((()<>)<()<>>)<(<>){<>{}}>]>(<{[<>{}]<{}<>>}>)>}(([<<{<>[]}<<
{[[{{([({<{{({{}()}<()[]>)[(<>{}){[]{}}]}}<(<{<><>}({}<>)><[{}()]<<>[]>>)>>{{<(<<>[]><{}[]>)>}[[<[[]
[([{<<({<{<[<[[][]][{}{}]>({<><>})]<({<>{}})>><<<<<>[]>(()<>)>({()[]})>>}<([{<{}<>>{{}<>}}([<><>]{
{[{({(([<[((<{[]()}{{}()}>{<()[]>[<>{}]})([[()[]]][{()()}([]{})])){(<{[]{}}[{}()]>[{<>[]}{{}()}]
[{[{({[{{<(<<([]<>)[()[]]><<[]{}><<>{}>>>[([<>[]](()<>)}([{}{}](<>[]))])>[<(<[{}[]]><<<><>>[[]()]>)[{[()(
[{<[<[[{{<{{((<>{})<[]{}>)(<<>[]>({}[]))}[<([]())<<><>>><<()<>>({}())>]}{<{[<>{}]}{([][]){[][]}}>({<[]{}>
({<((<([{{[[(([])[[]])[[<>{}][[]]]][[[[][]}{{}<>}]([{}{}][()<>])]]{[<[[][]]({}())>{[{}<>][<><>]}]}}}
{<<{({[[({{{<(<>())(<>{})>(({}[])(<>{}))}<<{[]{}}<[]>>>}{{(<<>{}><{}{}>)(<()()><[]()>)}{[({}
<[{{([<[{[((<[(){}][<>[]]>(<{}()>[<>[]]})[[(<>){<>()}]])<(([[][]]<<>{}>)){<[<><>][<>[]]><({}())
([(([((<<<{([{[]{}}])<[{()()}(()[])]{[()<>][<><>]}>>><{{[(()())<{}<>>]}(([[][]](<>[])))}((([
[<(({{{(<({[[[()[]][()[]]]{[<><>]{[]<>}}>[([()()]([][]))[[{}{}]{[]{}}]]}{[[[<>][{}[]]]][{[<>()][()<
[<[<<(([{{<<[((){})[<>{}]]{<[]()>}><{(()[])}((()[])[(){}])>>}}(({{[({}[])<()[]>]{[[][]]}}{(<<><>>)([(){}
[{[[<(({{({[{<()<>>((){})}(({}{}))](<[()<>]({}{})><<<>{}>(<>())>)}{[<<<>()>{<><>}><(()[])<{}[]>>]))}<{[(
<([[{{(([<<<<({}())([][])>[[[]{}]{[]<>}]>[{{()[]}<()[]>)[{{}[]}<[][]>]]>{<<[(){}](<>[])>[[{}[]]]>(
[[[[({[{<{(<{[[]<>]<(){}>}([(){}>[{}{}])>[{[{}()]{(){}}}])}>}]<[<(<{[[[]()]<{}<>>](<[]><(){
{(<[({{<<(<{{{{}{}}}<{<><>}<{}{}>>}<[<{}{}><()[]>]([{}[]]{<><>})>>)>>{[[([{<<><>>{()()}}<<<
<[({<([{[{[({<{}{}><{}[]>})([<()()>{<><>}]([()[]]{{}[]})>]({<{()()}<[]()>>[{[]()}({}())]})
<(([{(<([([([(()())[(){}]](<[]{}>[<><>]))(({()<>}<<>()>)(<{}[]>({}[])))][{[[[][]]](<<><>>)}
<{{(<{<[{((((({})({}<>))(({}())(<>[]))){{{[][]}[(){}]}<({}<>)<{}<>>>})){<({[{}()]<[]<>>}((()[])(<>{
{({({{{<((<{<{()[]}({}[])>{<{}<>>(()[])}}{([<>][[][]])}>)({({[<>()]<<>{}>}<[<>]([]{})>){([[]{
{[<((({<([<({[<>](<>)}((<>[]))){[[()()][<>]](([]){{}{}})}>])(<[[[[{}()]][({}<>){<><>}]]{<[
[<<<[(<<{({{<<()<>>({}{})><[<>[]]({}{})>}<{[(){}][(){}>}[[<>{}]{<>()}]>})[{[[[{}]<[]<>>][{(){}}]]<{[(
[{((<[{<({[{<[[]<>][{}[]]>}[(([][])<<>()>){<{}{}>[{}()]}]][[{{()[]}<[]{}>}(({}<>)<{}[]>)]{<<[]<>><()()>}}]})>
{[{{(<[{{{({{<{}{}><()>}[{[]()}{[]<>}]})}}[<{<[<{}()>[()[]]]<{{}<>}{[]}>>{(<{}{}><()()>)}}{{({[]()})([
{<{{({([[<(<[<[]{}>{{}()}]<({}{}){(){}}>><([()()]<(){}>){([]())(()[])}>){([([]<>)]<{<><>}[()<>]>){{<[]<>>[(
<[([(([[<[<[[<<>[]>[[]<>]]]<({[]<>}{<>{}})<[(){}]<[]<>>>>>[(<([]())>)[{[(){}](<>{})}<<[]<>>{<
{<[(<<[{<(<<{({}{})[(){}]}{[{}[]]{[]()}}>{<<<>{}>{[]{}}>}>[<(<{}<>>){<(){}><{}<>>}>(<{<><>}{{}{}}><((){})
([(<[(({{({[<([][])[(){}]>{[[]()]}]<[{[][]}{{}[]}](<[]<>>{{}{}])>})}}({[([{(()())[[][]]}<((){})
{[<[(<<{([({{<{}[]>(<>)}<(<>())<<><>>>}{(([]<>){<>()})})((({<>}<<>[]>)[([]()){{}()}]){<{<>()}>})](<<<[{}[
<((<[{[[[<<(<{()[]}(()[])>((<><>)[{}{}])){(<<>()>{[][]})[<[]{}><()[]>]}>[{<<()[]>([][])><{[]{}}
[{{({[<<{[<<{(()()){{}{}}}<[{}[]]{[]{}}>>>({({(){}}[<><>])({{}{}}<{}()>)}[{({}<>)<{}()>}])](([({()
<(({([<[{(([(({}[])([]()))<<()<>>{[]<>})]<<<[][]>>(<{}<>><[]<>>)>){<[{<>}[<>[]]]{<{}<>>{[]<>}}>[<[<>[]][<>
([{<<[[[[([[<[[]]{<>[]}>{(()[])<(){}>}]<[<()[]>(<>())]({[]()}[[]()])>]<{<{()<>}<()[]>><<<>>{{}{}}>}>)
({{<<({(([<{(([][])[()[]])}[[[()<>](())])><[([{}()][{}<>])<[[]{}](()<>)>]>]{(<<((){})<(){}>>({
[(<([(<{{<[[[{{}<>}{<>{}}]({()<>}<()[]>)]{[<<>[]>[{}<>]]<[{}[]](()[])>}]>}<{<<{{{}[]}((){})}>}}<{<[{()()}[<
{<<[({[{{{[[{(()())({})}][<<<>{}>([]())>]][(([{}()]{()[]}))<({<>[]}[{}()])>]}[<(<<<><>>[[]{}]>)<<
<{{{(([[[(([<(()())({}<>)>({[]{}}{(){}})]({{()}<[]()>}([[]{}]{()[]})))({{{()[]}<[]<>>}<[<>()]<<>[]}>}[{<[][]>
[(<{<(([([<<<{[]()}<{}>>)<<{<>{}}{[]}>{<<>()>{{}<>}}>>][{<{[{}[]]}(<<>()>)>[{(<><>)({}<>)}<
[[<<<[({[[<<((<><>){<><>})[(<><>)[<>()]]>{<[{}]>}>]]}<{<([<{<>{}}<<>()>>({[]<>}{()()})]({<{}{}>{(){}}}>)>[[(<
{({[([{[[([<((()[])(()[]))<<{}()>{()()}>>({({}[])}[[<>()}<{}[]>])]<([{()<>}{<>()}]([()](<>{})))<<((
<[((([<[<({{(<<>()>[<>[]])(([]()){[]<>))}(({()[]}<(){}>)[[[]()]<<><>>])}{[{{{}<>}[<>{}]}][(([]<>)((){}))]}
((<<[[[{((<<([[]<>][<>{}])[{<>{}}(()[])]>([{(){}}{{}<>}][({}{})([]())])>)<[{{{(){}}<[]{}>}(<[]<>>({}<>))
[[[{({{[[<[<<<<><>>[[]<>]>[<[]<>><<>()>]>({{<>{}}})]{(<<<>{}>{{}{}}><(()[])[{}{}]>)}><<[{[()[]]{
{[[{([[<<[[[{[()[]]{<>()}}({<><>})]{((<>[])[<>()])}]<(([[][]]<()[]>)){<({}<>)(<>{}}>{(<>{})(<><>)}}>]{([{[<><
[([[<[<({[{(([()()]{{}[]})[([]())(()())])[{{[]()}[<>{}]}({{}[]}[<>()])]}]<([<({}[]){[]()}>(<()()>[()<>])][(
<(<({{<(([[([[<>()]((){})]{(<>){<>[]}})[((<><>)({}{}))]]([<[{}[]]{[]<>}]][{<(){}><[]<>>}<[{}[
{[[({<[[{[({<([][]){<>()}>}(<(()()){()<>}>{<<>{}>}))<{<<()[]><{}<>>><({}<>)({}())>}((({}{})
(<(<({<<(<{[[<<>[]>({}{})][<[]()>{()()}]]}({{{<>[]}[<>{}]}({(){}}<()[]>)}}>)>{{{[({[[]()][[]{}]}(<<>[]>)){
({{{<(({[{{{{[{}<>][()<>]}<{()<>}>}[[({}())<[]()>][[<>[]]({}())]]}[[([{}()]({}{}))<([]())>]<{<{}[]>
[{([({<{(<[({{()()}(()<>)}(({}<>)<[]{}>)){([[][]]<(){}>)}]<<<<<>()><{}<>>>[((){})[[]{}]]><{<[]{}>>>>>)[(
{[(<({[<{<{{<{<>()}({}[])>(<{}[]>[<>()])}[[<[]<>>({})]{<<>{}><()[]>}]}>}[{{<{([]())[()()]}([[][]][
{[{{{{<<<{<(([[][]]){{[]{}}({}{})})[([()()]{[]()}){([]{})<{}[]>}]>[<(<[][]>{[]<>})[{{}()}(<>(
<[({{([[([{({{{}{}}<{}[]>}([(){}][<>{}]))}<{<[<>[]][[]<>]>[([][])]}([(<>{})[[][]]])>][{[{([]{})[[]<
<{{{<(([({{({<{}<>>[()()]}(([]){{}}))}}[(({{[]()}<{}<>>}[{[][]}<[]{}>])[<[()()](()())><(()){<>{}}>])[{<{[]<>}
{(<[{{[((<[[[[()()]([][])]([<>{}][<>()])]]>))<{(<<<([]{}){(){}}><<{}>{[][]}>>>{{(<<>>[(){}])<[<>()]{
{[{({{(<{({<({<>}<[][]>)([[]{}]{[][]})>[{(()())(()<>)}]][<[<()[]><{}{}>]({<>[]}<{}[]>)>[{<{}{}>[{}{
[{<{<({<<{[<((()<>)[{}()]){(<><>)<()<>>}>{(([]<>)[[]<>])<{()}[<><>]>}]}({[{{{}<>}({}())>(<{}<>>{{}()})][<
((<[{[[<[<(([(()[])<[]{}>](({}<>){<>[]}))<((()[])({}())){{()[]}[()<>]}>)[[((<>[])[<>()])[{{}<>}[{}{}]]]]><
[{<({{(([([[<<()()><<><>>>[<()[]>([]<>)]]{[{()<>}]((()[]){{}{}})}])[({[([]<>)][[<>[]]<{}{}>]}{[[[]
(<{(([{{<[([[<()<>>(<>)]<([])[(){}]>]){{[<[]<>>{{}{}}]}({[{}]}[{()[]}{{}<>}])}]>{<<{{<(){}>({}{}
({{(<{{[[<{<[{{}{}}{<>[]}][(<>{}>(()<>)]>}[{<{{}<>}><{()<>}<[]()>>}]>({[{{{}{}}<{}{}>}(((){})[[]<>])]({<[][]>
"#;
