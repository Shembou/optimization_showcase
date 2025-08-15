import http from 'k6/http';
import { check, sleep } from 'k6';

export let options = {
  vus: 50,             // number of virtual users
  duration: '30s',     // total test duration
  insecureSkipTLSVerify: true,
};

export default function () {
  const res = http.get('https://go_api.localhost/v1/users');

  check(res, {
    'status is 200': (r) => r.status === 200,
  });

  sleep(1); // pause 1s between requests
}
