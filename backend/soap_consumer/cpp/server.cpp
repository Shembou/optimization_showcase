#include "soapH.h"  // include the generated source code headers
#include "ns.nsmap" // include XML namespaces

int ns__add(struct soap *soap, int a, int b, int *result)
{
    *result = a + b;
    return SOAP_OK;
}

int ns__subtract(struct soap *soap, int a, int b, int *result)
{
    *result = a - b;
    return SOAP_OK;
}

int main()
{
    struct soap soap;
    soap_init(&soap);

    if (soap_bind(&soap, NULL, 8080, 100) < 0)
    {
        soap_print_fault(&soap, stderr);
        exit(1);
    }

    while (true)
    {
        if (soap_accept(&soap) >= 0)
        {
            soap_serve(&soap);
            soap_end(&soap);
        }
        else
        {
            soap_print_fault(&soap, stderr);
            exit(1);
        }
    }

    soap_done(&soap);
    return 0;
}