.PHONY: gen-client gen-server
gen-client:
	g++ -o client client.cpp soapC.cpp soapClient.cpp stdsoap2.cpp
gen-server:
	g++ server.cpp soapC.cpp soapServer.cpp -o server stdsoap2.cpp