package goparser

import (
	"encoding/json"
	"testing"

	"github.com/bytedance/sonic"
	jsoniter "github.com/json-iterator/go"
)

func BenchmarkSonic_Marshal(b *testing.B) {
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		_, err := sonic.Marshal(&twitter)
		if err != nil {
			b.Fatal(err)
		}
	}
}

func BenchmarkSonic_Unmarshal(b *testing.B) {
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		err := sonic.Unmarshal(data, &twitter)
		if err != nil {
			b.Fatal(err)
		}
	}
}

func BenchmarkEasyJson_Marshal(b *testing.B) {
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		_, err := json.Marshal(&twitterEasyJson)
		if err != nil {
			b.Fatal(err)
		}
	}
}

func BenchmarkEasyJson_Unmarshal(b *testing.B) {
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		err := json.Unmarshal(data, &twitterEasyJson)
		if err != nil {
			b.Fatal(err)
		}
	}
}

func BenchmarkJsoniter_Marshal(b *testing.B) {
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		_, err := jsoniter.Marshal(&twitterEasyJson)
		if err != nil {
			b.Fatal(err)
		}
	}
}

func BenchmarkJsoniter_Unmarshal(b *testing.B) {
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		err := jsoniter.Unmarshal(data, &twitterEasyJson)
		if err != nil {
			b.Fatal(err)
		}
	}
}
