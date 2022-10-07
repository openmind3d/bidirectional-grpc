package main

import (
	"context"
	"log"
	"time"

	proto "golang-grpc/proto"

	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
)

func main() {
	conn, err := grpc.Dial("localhost:8080", grpc.WithTransportCredentials(insecure.NewCredentials()))
	if err != nil {
		log.Fatal(err)
	}
	defer conn.Close()

	client := proto.NewPingPongServiceClient(conn)
	stream, err := client.Ping(context.Background())
	if err != nil {
		log.Fatal(err)
	}
	// ctx := stream.Context()
	go func() {
		for {
			message, err := stream.Recv()
			if err != nil {
				log.Fatal(err)
			}
			log.Println("recv:", message.Type)
		}
	}()
	for {
		if err := stream.Send(&proto.Message{Type: "ping from client"}); err != nil {
			log.Fatal(err)
		}

		time.Sleep(2 * time.Second)
	}
}
