const adminLogin = "admin";
const adminHashPassword = "8c6976e5b5410415bde908bd4dee15dfb167a9c873fc4bb8a81f6f2ab448a918";

export const checkAdmin = (userLogin: string, userHashPassword: string): boolean => {
    return userLogin == adminLogin && userHashPassword == adminHashPassword;
}