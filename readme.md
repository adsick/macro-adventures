## введение в декларативные макросы

синтаксис декларативного макроса
```
#[macro_export]
macro_rules! macro_name {
    (pattern1) => {branch1};
    (pattern2) => {branch2};
    ...
}

fn some_fn(){
    macro_name!(args) // macro invocation
}
```

Паттерны это шаблоны, по которым сопоставляется синтаксис аргумента макроса.
Аргумент макроса при вызове помещается в скобочки () [] {}

```
vec![1, 2, 3];
vec!{1, 2, 3};
vec!(1, 2, 3);
```

Паттерны состоят из ключевых слов, метапеременных и их повторений:

```
( $e:expr ) => { println!("expression: {}", stringify!($e)) }
```


```
( $a:expr, $b:expr ) => { $a + $b; }
```

логика работы macro_rules напоминает таковую у простого match выражения - аргумент макроса последовательно сопоставляется со всеми представленными паттернами и первый успешно сопоставленный код будет заменён блоком кода соответствующей ветки, после чего происходит выход из macro_rules.

### фрагменты

    item: an Item 
    block: a BlockExpression
    stmt: a Statement without the trailing semicolon
    pat_param: a PatternNoTopAlt
    pat: equivalent to pat_param
    expr: an Expression
    ty: a Type
    ident: an IDENTIFIER_OR_KEYWORD
    path: a TypePath style path
    tt: a TokenTree
    meta: an Attr, the contents of an attribute
    lifetime: a LIFETIME_TOKEN
    vis: a possibly empty Visibility qualifier
    literal: matches -?LiteralExpression

ходовые:

expr - выражение, ty, tt, block

### повторения
синтаксис:

```
$($name:frag_type)[delim]repetition
```
где name - имя переменной, frag_type - тип фрагмента из вышеперечисленных, delim - опциональный разделитель последовательности, repetition - вид повторения(?, *, +).

### примеры повторений:
Повторение с мета-переменной ```$v``` которая имеет "тип" expr - выражение и повторяется любое число раз (в том числе ноль), 
```
$($v:expr),*
```
этот паттерн сматчит например вот такое:
```
() OK
(1) OK
(1, 2, 3) OK
{1, 2, 3} OK
```
но не сматчит вот такое:
```
(1; 2; 3) ERR - неправильный разделитель
```

квалификаторы повторений:
```
    ? ноль или один
    * ноль или больше
    + один или больше
```

### использование повторений

посмотрите примеры кода в репозитории, рекомендуемый порядок просмотра: vec, map, list...

## основные приёмы
* возврат блока кода, возвращающего значение
* вложенные правила ("internal rules") - ветки формата (@rule_name ...rest of the pattern)
* итеративный парсинг с помощью паттерна "TT muncher" (см. маленькую книгу макросов)
имееет вид ```( $current:expr, ($tail:tt),*)=>{ println!("{}", stringify!($current)); macro_name!(tail); }```
* использование ```$(,)?``` на конце паттерна для поддержки trailing commas (```vec![1, 2, 3,]```)


## частые ошибки
* забытый знак $
* левая-правая ассоциативность вызовов
* неучёт жадности повторений: вот такое не сработает ```$($v:expr),*, $v:expr``` (а наоборот - сработает, так мы костыляем tail рекурсию)

## ссылки
* доклад Алексея Кладова: https://www.youtube.com/watch?v=L4wgbmmMXTM&t=3488s 
* статья в the rust programming language: https://doc.rust-lang.org/book/ch19-06-macros.html
* статья в the rust reference: https://doc.rust-lang.org/reference/macros-by-example.html
* статья на logrocket, заметил там несколько ошибок но в целом ок: https://blog.logrocket.com/macros-in-rust-a-tutorial-with-examples/
* "маленькая книга макросов" (the little book of rust macros): https://veykril.github.io/tlborm/ 
