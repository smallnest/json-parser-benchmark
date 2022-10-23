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
     Running benches/bench.rs (target/release/deps/bench-72246fe0c0b1d2fa)
twitter-serde/twitter-serde-marshal
                        time:   [9.0386 µs 9.0536 µs 9.0707 µs]
                        thrpt:  [1.6595 GiB/s 1.6626 GiB/s 1.6654 GiB/s]
Found 41 outliers among 500 measurements (8.20%)
  3 (0.60%) low mild
  11 (2.20%) high mild
  27 (5.40%) high severe
twitter-serde/twitter-serde-unmarshal
                        time:   [35.772 µs 35.861 µs 35.966 µs]
                        thrpt:  [428.58 MiB/s 429.83 MiB/s 430.90 MiB/s]
Found 31 outliers among 500 measurements (6.20%)
  9 (1.80%) high mild
  22 (4.40%) high severe
```
可以看到serde序列化只需要9微秒(μs), 反序列化只需要36微秒(μs)。

即使像sonic这样高度优化的Go序列化库，也比不上rust的序列化库，rust语言性能优势这么明显么？

## C++

- simdjson
- repaidjson