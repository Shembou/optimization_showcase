#include "soapH.h"
#include "ns.nsmap"

extern "C" int soap_add(int a, int b) {
    struct soap soap;
    soap_init(&soap);
    int result;
    if (soap_call_ns__add(&soap, "http://localhost:8080", "", a, b, &result) == SOAP_OK) {
        return result;
    }
    return -1; // Indicate error
}

extern "C" int soap_subtract(int a, int b) {
    struct soap soap;
    soap_init(&soap);
    int result;
    if (soap_call_ns__subtract(&soap, "http://localhost:8080", "", a, b, &result) == SOAP_OK) {
        return result;
    }
    return -1; // Indicate error
}