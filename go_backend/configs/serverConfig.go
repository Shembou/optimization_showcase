package configs

import (
	"database/sql"
	"flag"
	"log"
	"os"
	"strconv"

	"github.com/dgraph-io/ristretto/v2"
)

type Config struct {
	Port     int
	Env      string
	Dsn      string
	Logger   *log.Logger
	Database *sql.DB
	Cache    *ristretto.Cache[string, string]
}

func (config *Config) ConfigureServer() Config {
	var cfg Config
	cfg.Logger = log.New(os.Stdout, "", log.Ldate|log.Ltime)
	port, err := strconv.Atoi(os.Getenv("PORT"))
	if err != nil {
		cfg.Logger.Printf("Could not parse variable PORT %v", port)
		port = 50051
	}

	flag.IntVar(&cfg.Port, "port", port, "Api server port")
	flag.StringVar(&cfg.Env, "env", "dev", "Environment (dev|staging)")
	flag.StringVar(&cfg.Dsn, "db-dsn", os.Getenv("READINGLIST_DB_DSN"), "PostgreSQL DSN")
	flag.Parse()
	f, err := os.OpenFile("logs", os.O_RDWR|os.O_CREATE|os.O_APPEND, 0666)
	if err != nil {
		log.Fatalf("error opening file: %v", err)
	}
	defer f.Close()

	cfg.Database, err = sql.Open("postgres", cfg.Dsn)
	if err != nil {
		cfg.Logger.Fatal(err)
	}

	err = cfg.Database.Ping()
	if err != nil {
		cfg.Logger.Fatal(err)
	}

	cfg.Cache, err = ristretto.NewCache(&ristretto.Config[string, string]{
		NumCounters:            1e7,
		MaxCost:                1 << 30,
		BufferItems:            64,
		TtlTickerDurationInSec: 60,
	})

	if err != nil {
		cfg.Logger.Fatal("Failed to initialize cache. Exiting program")
	}

	cfg.Logger.Printf("database connection was established")
	return cfg
}
