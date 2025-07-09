const API_URL = "http://localhost:8080/api";

export function concatPath(path) {
    return `${API_URL}${path}`;
}
