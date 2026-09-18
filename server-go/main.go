// CyQuote API Server (Go)
// 单二进制语录服务：首次运行生成 config.json 与示例 data/quotes.jsonc。
package main

import (
	"encoding/json"
	"fmt"
	"math/rand"
	"net/http"
	"os"
	"path/filepath"
	"strconv"
	"strings"
	"time"
)

type Quote struct {
	Value  string `json:"value"`
	Author string `json:"author,omitempty"`
	From   string `json:"from,omitempty"`
}

type Config struct {
	Listen      string `json:"listen"`
	Port        int    `json:"port"`
	QuotesFile  string `json:"quotesFile"`
	AllowOrigin string `json:"allowOrigin"`
}

const defaultConfig = `{
  "listen": "127.0.0.1",
  "port": 9093,
  "quotesFile": "data/quotes.jsonc",
  "allowOrigin": "*"
}
`

const exampleCatalog = `// CyQuote 语录数据示例
// 顶层键为分类名，值为语录数组；每条包含 value（正文）、author（作者，可省略）、from（出处，可省略）。
// 支持 // 行注释与 /* 块注释 */，也容忍多余的尾逗号。
{
  "励志": [
    { "value": "路虽远，行则将至；事虽难，做则必成。" },
    { "value": "不积跬步，无以至千里；不积小流，无以成江海。", "author": "荀子", "from": "《劝学》" }
  ],
  "温柔": [
    { "value": "因为世界对我温柔，我就长成温柔的模样。", "author": "德谬歌", "from": "HSR" }
  ]
}
`

var catalog = map[string][]Quote{}
var categories = []string{}
var config Config
var total int
var random = rand.New(rand.NewSource(time.Now().UnixNano()))

func executableDir() string {
	path, err := os.Executable()
	if err != nil {
		return "."
	}
	return filepath.Dir(path)
}

func logLine(message string) {
	line := fmt.Sprintf("%s %s\n", time.Now().Format("2006-01-02 15:04:05"), message)
	if handle, err := os.OpenFile(filepath.Join(executableDir(), "cyquote.log"), os.O_CREATE|os.O_APPEND|os.O_WRONLY, 0o644); err == nil {
		_, _ = handle.WriteString(line)
		_ = handle.Close()
	}
	_, _ = os.Stdout.WriteString(line)
}

func loadConfig(base string) Config {
	path := filepath.Join(base, "config.json")
	if _, err := os.Stat(path); os.IsNotExist(err) {
		_ = os.WriteFile(path, []byte(defaultConfig), 0o644)
		logLine("已生成默认配置 config.json")
	}
	cfg := Config{Listen: "127.0.0.1", Port: 9093, QuotesFile: "data/quotes.jsonc", AllowOrigin: "*"}
	if data, err := os.ReadFile(path); err == nil {
		_ = json.Unmarshal(data, &cfg)
	}
	if cfg.Listen == "" {
		cfg.Listen = "127.0.0.1"
	}
	if cfg.Port <= 0 || cfg.Port > 65535 {
		cfg.Port = 9093
	}
	if cfg.QuotesFile == "" {
		cfg.QuotesFile = "data/quotes.jsonc"
	}
	if cfg.AllowOrigin == "" {
		cfg.AllowOrigin = "*"
	}
	return cfg
}

func ensureCatalogFile(path string) {
	if _, err := os.Stat(path); err == nil {
		return
	}
	_ = os.MkdirAll(filepath.Dir(path), 0o755)
	_ = os.WriteFile(path, []byte(exampleCatalog), 0o644)
	logLine("已生成示例语录文件 " + path)
}

func stripJSONC(input string) string {
	var out strings.Builder
	out.Grow(len(input))
	inString := false
	escaped := false
	for i := 0; i < len(input); i++ {
		char := input[i]
		if inString {
			out.WriteByte(char)
			if escaped {
				escaped = false
			} else if char == '\\' {
				escaped = true
			} else if char == '"' {
				inString = false
			}
			continue
		}
		if char == '"' {
			inString = true
			out.WriteByte(char)
			continue
		}
		if char == '/' && i+1 < len(input) {
			if input[i+1] == '/' {
				for i < len(input) && input[i] != '\n' {
					i++
				}
				if i < len(input) {
					out.WriteByte('\n')
				}
				continue
			}
			if input[i+1] == '*' {
				i += 2
				for i+1 < len(input) && !(input[i] == '*' && input[i+1] == '/') {
					i++
				}
				i++
				continue
			}
		}
		out.WriteByte(char)
	}
	cleaned := out.String()
	var result strings.Builder
	result.Grow(len(cleaned))
	inString = false
	escaped = false
	for i := 0; i < len(cleaned); i++ {
		char := cleaned[i]
		if inString {
			result.WriteByte(char)
			if escaped {
				escaped = false
			} else if char == '\\' {
				escaped = true
			} else if char == '"' {
				inString = false
			}
			continue
		}
		if char == '"' {
			inString = true
			result.WriteByte(char)
			continue
		}
		if char == ',' {
			next := i + 1
			for next < len(cleaned) && (cleaned[next] == ' ' || cleaned[next] == '\t' || cleaned[next] == '\n' || cleaned[next] == '\r') {
				next++
			}
			if next < len(cleaned) && (cleaned[next] == '}' || cleaned[next] == ']') {
				continue
			}
		}
		result.WriteByte(char)
	}
	return result.String()
}

func loadCatalog(path string) error {
	ensureCatalogFile(path)
	data, err := os.ReadFile(path)
	if err != nil {
		return err
	}
	parsed := map[string][]Quote{}
	if err := json.Unmarshal([]byte(stripJSONC(string(data))), &parsed); err != nil {
		return err
	}
	next := map[string][]Quote{}
	nextCategories := []string{}
	total = 0
	for category, quotes := range parsed {
		valid := make([]Quote, 0, len(quotes))
		for _, quote := range quotes {
			if strings.TrimSpace(quote.Value) == "" {
				continue
			}
			valid = append(valid, quote)
		}
		if len(valid) == 0 {
			continue
		}
		next[category] = valid
		nextCategories = append(nextCategories, category)
		total += len(valid)
	}
	catalog = next
	categories = nextCategories
	return nil
}

func respond(writer http.ResponseWriter, status int, body any) {
	writer.Header().Set("Access-Control-Allow-Origin", config.AllowOrigin)
	writer.Header().Set("Access-Control-Allow-Methods", "GET, OPTIONS")
	writer.Header().Set("Access-Control-Allow-Headers", "*")
	writer.Header().Set("Cache-Control", "no-store, max-age=0")
	writer.Header().Set("Content-Type", "application/json; charset=utf-8")
	writer.WriteHeader(status)
	_ = json.NewEncoder(writer).Encode(body)
}

func selectedQuotes(request *http.Request) []string {
	requested := strings.Split(request.URL.Query().Get("category"), ",")
	selected := []string{}
	for _, item := range requested {
		name := strings.TrimSpace(item)
		if name == "" {
			continue
		}
		if _, ok := catalog[name]; ok {
			selected = append(selected, name)
		}
	}
	if len(selected) == 0 {
		return categories
	}
	return selected
}

func handleQuote(writer http.ResponseWriter, request *http.Request) {
	selected := selectedQuotes(request)
	pool := []Quote{}
	for _, category := range selected {
		pool = append(pool, catalog[category]...)
	}
	if len(pool) == 0 {
		respond(writer, http.StatusNotFound, map[string]string{"error": "没有可用的语录"})
		return
	}
	quote := pool[random.Intn(len(pool))]
	respond(writer, http.StatusOK, map[string]any{
		"value":    quote.Value,
		"author":   quote.Author,
		"from":     quote.From,
		"category": selected,
		"source":   "CyQuote",
	})
}

func handleCategories(writer http.ResponseWriter) {
	respond(writer, http.StatusOK, map[string]any{"categories": categories})
}

func handleCount(writer http.ResponseWriter, request *http.Request) {
	requested := strings.TrimSpace(request.URL.Query().Get("category"))
	counts := map[string]int{}
	for category, quotes := range catalog {
		counts[category] = len(quotes)
	}
	if requested == "" {
		respond(writer, http.StatusOK, map[string]any{"total": total, "categories": counts})
		return
	}
	if strings.Contains(requested, ",") {
		respond(writer, http.StatusBadRequest, map[string]string{"error": "一次只能查询一个分类"})
		return
	}
	count, ok := counts[requested]
	if !ok {
		respond(writer, http.StatusNotFound, map[string]any{"error": "分类不存在", "category": requested})
		return
	}
	respond(writer, http.StatusOK, map[string]any{"category": requested, "count": count})
}

func handleHealth(writer http.ResponseWriter) {
	respond(writer, http.StatusOK, map[string]any{"status": "ok", "categories": len(categories), "quotes": total})
}

func route(writer http.ResponseWriter, request *http.Request) {
	if request.Method == http.MethodOptions {
		writer.Header().Set("Access-Control-Allow-Origin", config.AllowOrigin)
		writer.Header().Set("Access-Control-Allow-Methods", "GET, OPTIONS")
		writer.Header().Set("Access-Control-Allow-Headers", "*")
		writer.WriteHeader(http.StatusNoContent)
		return
	}
	if request.Method != http.MethodGet {
		respond(writer, http.StatusMethodNotAllowed, map[string]string{"error": "仅支持 GET"})
		return
	}
	path := strings.TrimSuffix(request.URL.Path, "/")
	switch path {
	case "", "/quote":
		handleQuote(writer, request)
	case "/categories":
		handleCategories(writer)
	case "/count":
		handleCount(writer, request)
	case "/health":
		handleHealth(writer)
	default:
		respond(writer, http.StatusNotFound, map[string]string{"error": "未知路径"})
	}
}

func main() {
	base := executableDir()
	config = loadConfig(base)
	if err := loadCatalog(filepath.Join(base, config.QuotesFile)); err != nil {
		logLine("载入语录失败：" + err.Error())
	}
	address := config.Listen + ":" + strconv.Itoa(config.Port)
	logLine(fmt.Sprintf("CyQuote 已启动：http://%s（%d 个分类 / %d 条语录）", address, len(categories), total))
	server := &http.Server{Addr: address, Handler: http.HandlerFunc(route)}
	if err := server.ListenAndServe(); err != nil {
		logLine("服务退出：" + err.Error())
	}
}
