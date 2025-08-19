package localhost.soap;

import java.util.List;

import io.quarkus.logging.Log;
import jakarta.inject.Inject;
import jakarta.jws.WebMethod;
import jakarta.jws.WebService;
import jakarta.persistence.EntityManager;
import jakarta.transaction.Transactional;
import localhost.soap.models.User;

@WebService(serviceName = "UserService")
public class UserServiceImpl implements UserService {
    @Inject
    EntityManager em;

    @WebMethod
    @Override
    @Transactional
    public List<User> getUsers() {
        try {
            List<User> users = em.createQuery("SELECT u FROM User u", User.class).getResultList();
            return users;
        } catch (Exception e) {
            Log.error("Error while getting users", e);
            return null;
        }
    }
}
