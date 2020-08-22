server:
  rsocket-cli -i "I am a Server" --server --debug tcp://localhost:7878

client:
  rsocket-cli --request -i "ping" --debug tcp://localhost:7878
