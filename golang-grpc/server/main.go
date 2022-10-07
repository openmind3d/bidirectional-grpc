package main

import (
	proto "golang-grpc/proto"
	"log"
	"net"
	"time"

	"google.golang.org/grpc"
)

type PingPongServer struct {
	proto.UnimplementedPingPongServiceServer
}

func (s *PingPongServer) Ping(stream proto.PingPongService_PingServer) error {
	go func() {
		for {
			if err := stream.Send(&proto.Message{Type: "ping from server"}); err != nil {
				log.Fatal(err)
			}
			time.Sleep(3 * time.Second)
		}
	}()
	for {
		ping, err := stream.Recv()
		if err != nil {
			log.Fatal(err)
		}
		log.Println("recv:", ping.Type)
		if err := stream.Send(&proto.Message{Type: "pong"}); err != nil {
			log.Fatal(err)
		}
	}
	return nil
}

func main() {
	listener, err := net.Listen("tcp", "localhost:8080")
	if err != nil {
		log.Fatal(err)
	}
	s := grpc.NewServer()
	proto.RegisterPingPongServiceServer(s, &PingPongServer{})
	log.Println("Server listening at:", listener.Addr())
	if err := s.Serve(listener); err != nil {
		log.Fatal(err)
	}
}
