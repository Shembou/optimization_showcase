package localhost.soap;

import java.util.List;

import jakarta.jws.WebMethod;
import jakarta.jws.WebService;
import localhost.soap.models.User;

@WebService(name = "UserService", serviceName = "UserService")
public interface UserService {
    @WebMethod
    List<User> getUsers();
}
