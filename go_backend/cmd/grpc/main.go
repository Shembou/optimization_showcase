package main

import (
	"context"
	"crypto/tls"
	"log"
	"net"

	pb "github.com/Shembou/optimization_showcase/backend/cmd/grpc/user"
	"github.com/Shembou/optimization_showcase/backend/configs"
	"github.com/Shembou/optimization_showcase/backend/internal/data"
	_ "github.com/lib/pq"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials"
)

type server struct {
	pb.UnimplementedUserServer
	app *application
}

type application struct {
	config configs.Config
	models data.Models
	logger *log.Logger
}

func (s *server) UserRPC(ctx context.Context, in *pb.Empty) (*pb.UsersList, error) {
	users, err := s.app.models.Users.GetAll()
	if err != nil {
		s.app.logger.Fatalf("Database query execution failed")
		return nil, err
	}
	var mappedUsers []*pb.UserMessage
	for _, u := range users {
		mappedUsers = append(mappedUsers, &pb.UserMessage{
			Id:       int32(u.ID),
			Name:     u.Name,
			Language: u.Language,
			Bio:      u.Bio,
			Version:  int32(u.Version),
		})
	}
	s.app.logger.Println("Serving grpc data to client")
	return &pb.UsersList{Users: mappedUsers}, nil
}

func loadTLSCredentials() (credentials.TransportCredentials, error) {
	serverCert, err := tls.LoadX509KeyPair("certs/cert.pem", "certs/key.pem")
	if err != nil {
		return nil, err
	}

	tlsConfig := &tls.Config{
		Certificates: []tls.Certificate{serverCert},
		ClientAuth:   tls.NoClientCert,
	}

	return credentials.NewTLS(tlsConfig), nil
}

func main() {
	cfg := &configs.Config{}
	configuration := cfg.ConfigureServer()
	defer cfg.Database.Close()
	defer cfg.Cache.Close()

	app := application{
		config: configuration,
		logger: configuration.Logger,
		models: data.NewModels(configuration.Database, configuration.Cache),
	}

	lis, err := net.Listen("tcp", ":50051")
	if err != nil {
		log.Fatalf("failed to listen: %v", err)
	}

	tlsCredentials, err := loadTLSCredentials()
	if err != nil {
		log.Fatal("cannot load TLS credentials: ", err)
	}

	grpcServer := grpc.NewServer(
		grpc.Creds(tlsCredentials),
		grpc.MaxSendMsgSize(1024*1024*50),
		grpc.MaxRecvMsgSize(1024*1024*50),
	)
	pb.RegisterUserServer(grpcServer, &server{app: &app})

	log.Println("gRPC server listening on :50051")
	if err := grpcServer.Serve(lis); err != nil {
		log.Fatalf("failed to serve: %v", err)
	}
}
