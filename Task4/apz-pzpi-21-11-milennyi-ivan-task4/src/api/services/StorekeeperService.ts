import { Storekeeper } from "../../models/Storekeeper";
import { AuthService } from "../interfaces/IAuthService";
import { IStorekeeperService } from "../interfaces/IStorekeeperService";

export class StorekeeperService implements IStorekeeperService, AuthService<Storekeeper>{
    create(item: Storekeeper): Promise<Storekeeper> {
        throw new Error("Method not implemented.");
    }
    delete(itemId: number): Promise<void> {
        throw new Error("Method not implemented.");
    }
    update(item: Storekeeper): Promise<Storekeeper> {
        throw new Error("Method not implemented.");
    }
    getAll(): Promise<Storekeeper[]> {
        throw new Error("Method not implemented.");
    }
    getById(id: number): Promise<Storekeeper | null> {
        throw new Error("Method not implemented.");
    }
    checkUsername(user: Storekeeper): Promise<boolean> {
        throw new Error("Method not implemented.");
    }
    authorize(username: string, passwordHash: string): Promise<Storekeeper | null> {
        throw new Error("Method not implemented.");
    }
    
}