import { Shepherd } from "../../models/Shepherd";
import { Storekeeper } from "../../models/Storekeeper";
import { AuthService } from "../interfaces/IAuthService";
import { IShepherdService } from "../interfaces/IShepherdService";

export class ShepherdService implements IShepherdService, AuthService<Shepherd>{
    create(item: Shepherd): Promise<Shepherd> {
        throw new Error("Method not implemented.");
    }
    delete(itemId: number): Promise<void> {
        throw new Error("Method not implemented.");
    }
    update(item: Shepherd): Promise<Shepherd> {
        throw new Error("Method not implemented.");
    }
    getAll(): Promise<Shepherd[]> {
        throw new Error("Method not implemented.");
    }
    getById(id: number): Promise<Shepherd | null> {
        throw new Error("Method not implemented.");
    }
    checkUsername(user: Shepherd): Promise<boolean> {
        throw new Error("Method not implemented.");
    }
    authorize(username: string, passwordHash: string): Promise<Shepherd | null> {
        throw new Error("Method not implemented.");
    }
   
}
