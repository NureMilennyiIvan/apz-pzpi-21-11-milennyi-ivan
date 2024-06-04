import { AuthUser } from "../../utils/AuthUser";

export interface IUserProps {
    user: AuthUser;
    setUser: React.Dispatch<React.SetStateAction<AuthUser>>;
}