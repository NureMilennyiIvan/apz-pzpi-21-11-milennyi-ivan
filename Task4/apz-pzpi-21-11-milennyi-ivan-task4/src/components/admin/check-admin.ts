const adminLogin = "admin";
const adminHashPassword = "";

export const checkAdmin = (userLogin: string, userHashPassword: string): boolean => {
    return userLogin === adminLogin && userHashPassword === adminHashPassword;
}