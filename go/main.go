package main

import (
    "fmt"
    "log"
    "net/http"
    "net/http/httputil"

    "github.com/k0kubun/pp"
)

func handler(w http.ResponseWriter, r *http.Request) {
    dump, err := httputil.DumpRequest(r, true)
    if err != nil {
        http.Error(w, fmt.Sprint(err), http.StatusInternalServerError)
        return
    }
    fmt.Println(string(dump))
    fmt.Fprintf(w, "<html><body>hello</body></html>\n")
}

func handlerDigest(w http.ResponseWriter, r *http.Request) {
    pp.Printf("URL: %s\n", r.URL.String())
    pp.Printf("Query: %s\n", r.URL.Query());

    defer r.Body.Close()
    // TODO: unimplemented
}

func main() {
    var httpServer http.Server
    http.HandleFunc("/", handler)
    log.Println("start http listening :18888")
    httpServer.Addr = ":18888"
    log.Println(httpServer.ListenAndServe())
}
