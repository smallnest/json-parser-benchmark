# 测试 各种编程语言的json序列化反序列化能力

当前仅支持了 Go 语言和 Rust 语言的代码。

使用 [Twitter.json](testdata/twitter.json), 这也是 [simdjson](https://github.com/simdjson/simdjson) 和字节跳动[sonic](https://github.com/bytedance/sonic)测试的场景之一。

## Go

```go
BenchmarkSonic_Marshal
BenchmarkSonic_Marshal-2        	   71607	     15908 ns/op	   10668 B/op	       4 allocs/op
BenchmarkSonic_Unmarshal
BenchmarkSonic_Unmarshal-2      	   14989	     81607 ns/op	   17769 B/op	       3 allocs/op
BenchmarkEasyJson_Marshal
BenchmarkEasyJson_Marshal-2     	   15283	     77838 ns/op	   21405 B/op	      66 allocs/op
BenchmarkEasyJson_Unmarshal
BenchmarkEasyJson_Unmarshal-2   	    7549	    146113 ns/op	   22405 B/op	     123 allocs/op
BenchmarkJsoniter_Marshal
BenchmarkJsoniter_Marshal-2     	   32162	     40143 ns/op	   20214 B/op	      64 allocs/op
BenchmarkJsoniter_Unmarshal
BenchmarkJsoniter_Unmarshal-2   	    9399	    134121 ns/op	   26194 B/op	     444 allocs/op
```

看一看到Sonic的序列化和反序列化d都非常优秀。这序列化反序列化`twitter.json`时，序列化平均需要16微秒(μs)， 反序列化需要82微秒(μs)。

相比较而言，大家常用Jsoniter、EasyJson耗时是它的两到三倍，所以如果你的程序不得不使用JSON的话，如果想提升性能，可以考虑使用sonic替换。

## Rust

Rust常用的Json序列化反序列化框架是[serde-json](https://github.com/serde-rs/json)。

虽然一种常见的方式是解析成成JsonValue的方式，但是我们还是用和Go语言相匹配的方式，解析成struct。

```rust

```



## C++

- simdjson
- repaidjson