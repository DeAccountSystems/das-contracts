# 自定义价格配置结构


## 价格规则

> 如果没有任何一条价格规则匹配成功，那么账户必须被当作不可注册账户处理。

一个子账户只能拥有一份价格规则配置，一份配置中可以同时存放多条规则，按照遍历数组的顺序执行，第一个匹配成功的价格规则就会作为当前账户的定价。
单条价格配置的字段如下：

- name ，utf-8 字符串类型，当前价格配置的名称，不能为空；
- note ，utf-8 字符串类型，当前价格规则的说明信息，可以为空；
- price ，整型，当前价格规则匹配成功后的账户价格，单位为 USD，与合约中其他地方相同，1 USD 存储为 `1_000_000` ；
- status ，整形，当前规则的状态 `0x00` 为关闭状态，`0x01` 为开启状态；
- ast ，当前价格执行匹配时的抽象语法树，最终计算结果必须为 `bool` 类型，详见下文的抽象语法树一节；

### JSON 存储结构

```json
[
    {
        "index": 0,
        "name": "...",
        "note": "...",
        "price": 0,
        "status": 0,
        "ast": { ... }
    }
]
```

### Molecule 存储结构

```
table SubAccountRule {
    index: Uint32,
    name: Bytes,
    note: Bytes,
    price: Uint64, // 必定为 0
    ast: ASTExpression,
    status: Uint8,
}

vector SubAccountRules <SubAccountRule>;
```


## 保留账户名规则

一个子账户只能拥有一份保留账户名配置，一份配置中可以同时存放多条规则，按照遍历数组的顺序执行，一旦匹配成功就将账户当作保留账户。
单条规则配置的字段如下：

- name ，utf-8 字符串类型，当前价格配置的名称，不能为空；
- note ，utf-8 字符串类型，当前价格规则的说明信息，可以为空；
- status ，整形，当前规则的状态 `0x00` 为关闭状态，`0x01` 为开启状态；
- ast ，当前价格执行匹配时的抽象语法树，最终计算结果必须为 `bool` 类型，详见下文的抽象语法树一节；

### JSON 存储结构

> 复用**价格规则**的数据结构，不过 price 必定为 0 。

```json
[
    {
        "index": 0,
        "name": "...",
        "note": "...",
        "price": 0,
        "status": 0,
        "ast": { ... }
    }
]
```

### Molecule 存储结构

> 复用**价格规则**的数据结构，不过 price 必定为 0 。

```
table SubAccountRule {
    index: Uint32,
    name: Bytes,
    note: Bytes,
    price: Uint64,
    ast: ASTExpression,
    status: Uint8,
}

vector SubAccountRules <SubAccountRule>;
```


## 抽象语法树

抽象语法树是一种用于描述配置规则的结构，可以简单的理解为一种基于配置文件的编程语言。

### expression

在本语法树的设计中，一切皆为 expression ，而 expression 又分为以下三类：

- operator ，此类 expression 会先对其参数进行计算得到下面的 **value** ，然后根据预定义的逻辑运算规则进行计算，并返回 `true` 或 `false`；
- function ，此类 expression 会先对其参数进行计算得到下面的 **value** ，然后根据自定义的函数逻辑进行计算，并返回 `true` 或 `false`；
- variable ，此类 expression 代表内置变量，如账户名、账户长度等信息；
- value ，此类 expression 代表 `int, binary, string` 等类型，必须结合 **operator** 或者 **funtion** 使用；

### operator

预定义的 operator 有以下几种 `symbol`：

- `and` ，表示 expression 之间为且关系，这种 operator 能接受多个 expression 且 expression 的类型必须为 `bool`；
- `or` ，表示 expression 之间为或关系，这种 operator 能接受多个 expression 且 expression 的类型必须为 `bool`；
- `not` ，表示对 expression 取非关系，这种 operator 只能接受 1 个 expression 且 expression 的类型必须为 `bool`；
- `>` ，表示对两个 expression 进行大于对比，这种 operator 只能接受 2 个 expression 且 expression 的类型必须为 `uint`，第一个 expression 在左，第二个 expression 在右进行对比；
- `>=` ，表示对两个 expression 进行大于等于对比，这种 operator 只能接受 2 个 expression 且 expression 的类型必须为 `uint`，第一个 expression 在左，第二个 expression 在右进行对比；
- `< ` ，表示对两个 expression 进行小于对比，这种 operator 只能接受 2 个 expression 且 expression 的类型必须为 `uint`，第一个 expression 在左，第二个 expression 在右进行对比；
- `<=` ，表示对两个 expression 进行小于等于对比，这种 operator 只能接受 2 个 expression 且 expression 的类型必须为 `uint`，第一个 expression 在左，第二个 expression 在右进行对比；
- `==` ，表示对两个 expression 进行等于对比，这种 operator 只能接受 2 个 expression 且 expression 的类型必须为 `uint`，第一个 expression 在左，第二个 expression 在右进行对比；

所有的 operator 执行结束后返回值都必须为 `bool` 类型的 value 。

### function

预定义的 function 有以下几种：

#### include_chars

检查账户名中是否包含特定字符，如果包含则返回 true ，反之返回 false ：

```
fn include_chars(account: string, chars: string[]) -> bool;
```

#### include_words

检查账户名中是否包含特定字符串，如果包含则返回 true ，反之返回 false ：

```
fn include_words(account: string, words: string[]) -> bool;
```

#### starts_with

检查账户名中是否以某些字符开头，如果是则返回 true ，反之返回 false ：

```
fn starts_with(account: string, words: string[]) -> bool;
```

#### ends_with

检查账户名中是否以某些字符结尾，如果是则返回 true ，反之返回 false ：

```
fn ends_with(account: string, words: string[]) -> bool;
```

#### only_include_charset

检查账户名中的字符是否仅包含特定字符集，如果包含则返回 true ，反之返回 false ：

```
fn only_include_charset(account_chars: account_chars, charset: charset_type) -> bool;
```

#### include_charset

检查账户名中的字符是否包含特定字符集，如果包含则返回 true ，反之返回 false ：

```
fn include_charset(account_chars: account_chars, charset: charset_type) -> bool;
```

#### in_list

检查账户名是否存在于列表中，如果是则返回 true ，反之返回 false ：

```
fn in_list(account: string, account_list: binary[]) -> bool;
```

- account_list，为使用 account 计算 account ID 后组成的数组；



### variable

预定义的内置变量有以下几种：

- account ，类型 string，代表账户名的 utf-8 字符串，包含后缀部分，比如 `xxxxx.bit` 的 account 变量就是 `xxxxx.bit`，再比如 `xxxxx.yyy.bit` 的 account 变量就是 `xxxxx.yyy.bit`；
- account_chars ，类型 string[]，代表账户名的 AccountChars 数据结构，其包含了账户名中每一个字符的所有信息，该结构体和其他地方一样不含后缀部分；
- account_length ，类型 uint32，代表账户名的字符长度，**也就是 AccountChars 数据结构长度**；

### value

预定义的 value 类型有以下几种：

- bool
- uint8 ，不同的 uint 类型之间可以转换为更大的类型后进行大小比较
- uint32 ，不同的 uint 类型之间可以转换为更大的类型后进行大小比较
- uint64 ，不同的 uint 类型之间可以转换为更大的类型后进行大小比较
- binary ，对应其他语言中的 `Buffer`、`Byte` 等类型，其在 json 中的存储方式为含 `0x` 前缀的 hex 字符串
- binary[] ，对应其他语言中的 `Buffer`、`Byte` 等类型，其在 json 中的存储方式为含 `0x` 前缀的 hex 字符串数组
- string ，utf-8 编码的字符串类型
- string[] ，utf-8 编码的字符串数组类型
- charset_type ，以 utf-8 字符串存放的字符集类型枚举值，可用值有：
    - Emoji
    - Digit
    - En
    - ZhHans
    - ZhHant
    - Ja
    - Ko
    - Ru
    - Tr
    - Th
    - Vi

> 因为部分语言解析 JSON 中的 uint64 类型时存在溢出的可能，因此在 JSON 中存放数字时支持 string 的形式数字，并且可以使用 `_` 作为提高可读性的分隔符，比如 `1 000 000 000` 可以写成 `1_000_000_000` 或 `10_00000000` ，`_` 在最终解析为数字时都将被忽略，无任何实际意义。


### JSON 存储结构

```json
{
    "type": "operator",
    "symbol": "and", // and|or|not|...
    "expressions": [
        {expression},
        {expression},
        ...
    ]
}

{
    "type": "function",
    "name": "in_list", // include_chars|include_words|only_include_charset|in_list
    "arguments": [
        {expression},
        {expression},
        {expression},
        ...
    ]
}

{
    "type": "variable",
    "name": "account" // account|account_chars|account_length
}

{
    "type": "value",
    "value_type": "bool", // bool|unit8|uint32|...
    "value": {value}
}
```

### Molecule 存储结构

```
// Because the molecule do not support recursive type, we can not use union here.
table ASTExpression {
    // Indicate the real type of expression field:
    // - 0x00 ASTOperator
    // - 0x01 ASTFunction
    // - 0x02 ASTVariable
    // - 0x03 ASTValue
    expression_type: byte,
    expression: Bytes,
}

vector ASTExpressions <ASTExpression>;

table ASTOperator {
    // Indicate the operator type:
    // - 0x00 `not`
    // - 0x01 `and`
    // - 0x02 `or`
    // - 0x03 `>`
    // - 0x04 `>=`
    // - 0x05 `<
    // - 0x06 `<=`
    // - 0x07 `==`
    symbol: byte,
    expressions: ASTExpressions,
}

table ASTFunction {
    // Indicate the function name:
    // - 0x00 `include_chars`
    // - 0x01 `include_words`
    // - 0x02 `only_include_charset`
    // - 0x03 `in_list`
    // - 0x04 `include_charset`
    // - 0x05 `starts_with`
    // - 0x06 `ends_with`
    name: byte,
    arguments: ASTExpressions,
}

table ASTVariable {
    // Indicate the variable name:
    // - 0x00 `account`
    // - 0x01 `account_chars`
    // - 0x02 `account_length`
    name: byte,
}

table ASTValue {
    // Indicate the value type
    // - 0x00 bool
    // - 0x01 uint8
    // - 0x02 uint32
    // - 0x03 uint64
    // - 0x04 binary
    // - 0x05 binary[]
    // - 0x06 string
    // - 0x07 string[]
    // - 0x08 charset_type
    value_type: byte,
    value: Bytes,
}
```

在 `ASTValue.value` 中，根据存储的类型不同分别对应以下 molecule 类型：

- bool => `byte` ，注意这里的 `byte` 并未写错大小写，而是因为它就是 molecule 编码的基本类型；
- uint8 => `Uint8`
- uint32 => `Uint32`
- uint64 => `Uint64`
- binary => `Bytes`
- binary[] => `BytesVec`
- string => `Bytes`
- string[] => `BytesVec`
- charset_type => `Uint32`


## 实际结构示例

### 按照长度定价

```json
[
    {
        "name": "1 位账户",
        "note": "",
        "price": 100000000, // 100 USD
        "ast": {
            "type": "operator",
            "symbol": "==",
            "expressions": [
                {
                    "type": "variable",
                    "name": "account_length"
                },
                {
                    "type": "value",
                    "value_type": "uint8",
                    "value": 1
                }
            ]
        }
    },
    {
        "name": "2 位账户",
        "note": "",
        "price": 10000000, // 10 USD
        "ast": {
            "type": "operator",
            "symbol": "==",
            "expressions": [
                {
                    "type": "variable",
                    "name": "account_length"
                },
                {
                    "type": "value",
                    "value_type": "uint",
                    "value": 2
                }
            ]
        }
    }
    ...
    {
        "name": "8 位及以上账户",
        "note": "",
        "price": 100000, // 0.1 USD
        "ast": {
            "type": "operator",
            "symbol": ">=",
            "expressions": [
                {
                    "type": "variable",
                    "name": "account_length"
                },
                {
                    "type": "value",
                    "value_type": "uint",
                    "value": 8
                }
            ]
        }
    }
]
```

### 按照长度及字符集定价

```json
[
    {
        "name": "1 位数字账户",
        "note": "",
        "price": 100000000, // 100 USD
        "ast": {
            "type": "operator",
            "symbol": "and",
            "expressions": [
                {
                    "type": "operator",
                    "symbol": "==",
                    "expressions": [
                        {
                            "type": "variable",
                            "name": "account_length"
                        },
                        {
                            "type": "value",
                            "value_type": "uint",
                            "value": 1
                        }
                    ]
                },
                {
                    "type": "function",
                    "name": "only_include_charset",
                    "arguments": [
                        {
                            "type": "variable",
                            "name": "account_charts"
                        },
                        {
                            "type": "value",
                            "value_type": "charset_type",
                            "value": "Digit"
                        }
                    ]
                }
            ]
        }
    },
]
```

### 按照是否含有特定字符定价

```json
[
    {
        "name": "特殊字符账户",
        "note": "",
        "price": 100000000, // 100 USD
        "ast": {
            "type": "function",
            "name": "include_chars",
            "arguments": [
                {
                    "type": "variable",
                    "name": "account"
                },
                {
                    "type": "value",
                    "value_type": "string[]",
                    "value": [
                        "⚠️",
                        "❌",
                        "✅"
                    ]
                }
            ]
        }
    },
]
```

### 按照白名单定价

```json
[
    {
        "name": "特殊账户",
        "note": "",
        "price": 10000000, // 10 USD
        "ast": {
            "type": "function",
            "name": "in_list",
            "arguments": [
                {
                    "type": "variable",
                    "name": "account"
                },
                {
                    "type": "value",
                    "value_type": "binary[]",
                    "value": [
                        "0x...",
                        "0x...",
                        ...
                    ]
                },
            ]
        }
    },
]
```

## AST 解析示例代码

```typescript
type SubAccountRule = {
    index: number,
    name: string,
    note: string,
    price: number,
    ast: Expression,
}

enum ExpressionType {
    Operator = 'operator',
    Function = 'function',
    Variable = 'variable',
    Value = 'value',
}

enum VairableName {
    Account = 'account',
    AccountChars = 'account_chars',
    AccountLength = 'account_length',
}

enum ValueType {
    Bool = 'bool',
    Uint8 = 'uint8',
    Uint32 = 'uint32',
    Uint64 = 'uint64',
    Binary = 'binary',
    String = 'string',
    StringArray = 'string[]',
    CharsetType = 'charset_type',
}

type Expression = OperatorExpr | FunctionExpr | VariableExpr | ValueExpr

type OperatorExpr = {
    type: ExpressionType.Operator,
    symbol: string,
    expressions: Expression[],
}

type FunctionExpr = {
    type: ExpressionType.Function,
    name: string,
    arguments: Expression[],
}

type VariableExpr = {
    type: ExpressionType.Variable,
    name: string,
}

type ValueExpr = {
    type: ExpressionType.Value,
    value_type: ValueType,
    value: boolean | number | string | string[],
}

function handleExpression(expr: Expression, account_chars): ValueExpr {
    let ret;
    switch(expr.type) {
        case 'operator':
            ret = handleOperator(expr, account_chars)
            break
        case 'function':
            ret = handleFunction(expr, account_chars)
            break
        case 'variable':
            ret = handleVariable(expr, account_chars)
            break
        case 'value':
            ret = expr
            break
        default:
            throw new Error('Unimplement expression found')
    }
    return ret
}

function handleOperator(operator: OperatorExpr, account_chars): ValueExpr {
    let ret: ValueExpr;
    switch(operator.symbol) {
        case "and":
            ret = AndOperator(operator.expressions, account_chars)
            break
        case "or":
            ret = OrOperator(operator.expressions, account_chars)
            break
        case "==":
            ret = EqualOperator(operator.expressions, account_chars)
            break
        // TODO more operator handler functions here ...
        default:
            throw new Error('Unimplement operator found')
    }

    return ret
}

function AndOperator(expressions: Expression[], account_chars): ValueExpr {
    for (let expr of expressions) {
        let ret = handleExpression(expr, account_chars)
        if (ret.type == 'value' && ret.value_type == 'bool') {
            if (ret.value) {
                continue
            } else {
                return {
                    type: ExpressionType.Value,
                    value_type: ValueType.Bool,
                    value: false
                }
            }
        } else {
            throw new Error('Expression type error, expected boolean')
        }
    }

    return {
        type: ExpressionType.Value,
        value_type: ValueType.Bool,
        value: true
    }
}

function OrOperator(expressions: Expression[], account_chars): ValueExpr {
    for (let expr of expressions) {
        let ret = handleExpression(expr, account_chars)
        if (ret.type == 'value' && ret.value_type == 'bool') {
            if (ret.value) {
                return {
                    type: ExpressionType.Value,
                    value_type: ValueType.Bool,
                    value: true
                }
            }
        } else {
            throw new Error('Expression type error, expected boolean')
        }
    }

    return {
        type: ExpressionType.Value,
        value_type: ValueType.Bool,
        value: false
    }
}

function EqualOperator(expressions: Expression[], account_chars): ValueExpr {
    if (expressions.length !== 2) {
        throw new Error('The == operator must accept 2 expressions')
    }

    let left = handleExpression(expressions[0], account_chars)
    if (![ValueType.Uint8, ValueType.Uint32, ValueType.Uint64].includes(left.value_type)) {
        throw new Error('The final value of == operator expression must be uint type')
    }

    let right = handleExpression(expressions[1], account_chars)
    if (![ValueType.Uint8, ValueType.Uint32, ValueType.Uint64].includes(right.value_type)) {
        throw new Error('The final value of == operator expression must be uint type')
    }

    return {
        type: ExpressionType.Value,
        value_type: ValueType.Bool,
        value: left.value === right.value
    }
}

function handleFunction(functionExpr: FunctionExpr, account_chars): ValueExpr {
    let ret: ValueExpr;
    switch(functionExpr.name) {
        case "include_chars":
            ret = includeChars(functionExpr.arguments, account_chars)
            break
        case "only_include_charset":
            ret = includeCharset(functionExpr.arguments, account_chars)
            break
        // TODO more operator handler functions here ...
        default:
            throw new Error('Unimplement function found')
    }

    return ret
}

function includeChars(args: Expression[], account_chars): ValueExpr {
    // TODO to be implement ...
    return {
        type: ExpressionType.Value,
        value_type: ValueType.Bool,
        value: true
    }
}

function includeCharset(args: Expression[], account_chars): ValueExpr {
    // TODO to be implement ...
    return {
        type: ExpressionType.Value,
        value_type: ValueType.Bool,
        value: true
    }
}

function handleVariable(variable: VariableExpr, account_chars): ValueExpr {
    switch(variable.name) {
        case 'account':
            // TODO to be implement ...
            return {
                type: ExpressionType.Value,
                value_type: ValueType.Uint32,
                value: 4
            }
        case 'account_chars':
            // TODO to be implement ...
            return {
                type: ExpressionType.Value,
                value_type: ValueType.StringArray,
                value: [ '2️⃣', '0️⃣', '7️⃣', '7️⃣' ]
            }
        case 'account_length':
            // TODO to be implement ...
            return {
                type: ExpressionType.Value,
                value_type: ValueType.Uint8,
                value: 4
            }
        default:
            throw new Error('Unsupported variable')
    }
}

function findSubAccountRule(subAccountRules: SubAccountRule[], account_chars): SubAccountRule {
    for (let subAccountRule of subAccountRules) {
        let val = handleExpression(subAccountRule.ast, account_chars)
        if (val.value_type !== ValueType.Bool) {
            throw new Error('AST returned invalid value')
        } else{
            if (val.value) {
                return subAccountRule
            } else {
                continue
            }
        }
    }

    throw new Error('Can not find any price for the account')
}

const subAccountRules: SubAccountRule[] = [
    {
        "name": "4 位 emoji 账户",
        "note": "",
        "price": 100000000, // 100 USD
        "ast": {
            "type": ExpressionType.Operator,
            "symbol": "and",
            "expressions": [
                {
                    "type": ExpressionType.Operator,
                    "symbol": "==",
                    "expressions": [
                        {
                            "type": ExpressionType.Variable,
                            "name": "account_length"
                        },
                        {
                            "type": ExpressionType.Value,
                            "value_type": ValueType.Uint8,
                            "value": 4
                        }
                    ]
                },
                {
                    "type": ExpressionType.Function,
                    "name": "only_include_charset",
                    "arguments": [
                        {
                            "type": ExpressionType.Variable,
                            "name": "account_charts"
                        },
                        {
                            "type": ExpressionType.Value,
                            "value_type": ValueType.CharsetType,
                            "value": "Emoji"
                        }
                    ]
                }
            ]
        }
    },
]

const accountChars = [
    {
        char_set: 'emoji',
        chars: '2️⃣'
    },
    {
        char_set: 'emoji',
        chars: '0️⃣'
    },
    {
        char_set: 'emoji',
        chars: '7️⃣'
    },
    {
        char_set: 'emoji',
        chars: '7️⃣'
    },
]

let subAccountRule = findSubAccountRule(subAccountRules, accountChars)

console.log(subAccountRule)
```