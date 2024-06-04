import { UserRole } from "./UserRole";

export class AuthUser{
    private id: number | null;
    private role: UserRole;

    public get Id(): number | null {
        return this.id;
    }

    public set Id(value: number | null) {
        this.id = value;
    }

    public get Role(): UserRole {
        return this.role;
    }
    public set Role(role: UserRole) {
        this.role = role;
    }

    constructor (id: number | null = null, role: UserRole){
        this.id = id,
        this.role = role
    }
}