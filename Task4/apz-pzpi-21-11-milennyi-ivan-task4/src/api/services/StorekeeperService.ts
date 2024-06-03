import axios from "axios";
import { Storekeeper } from "../../models/Storekeeper";
import { API_URL } from "../../utils/config";
import { AuthService } from "../interfaces/IAuthService";
import { IStorekeeperService } from "../interfaces/IStorekeeperService";

export class StorekeeperService implements IStorekeeperService, AuthService<Storekeeper>{
    private static STOREKEEPER_URLS = {
        GET_ALL: `${API_URL}/storekeeper`,
        GET_BY_ID: (id: number) => `${API_URL}/storekeeper/${id}`,
        CREATE: `${API_URL}/storekeeper/create`,
        DELETE: (id: number) => `${API_URL}/storekeeper/delete/${id}`,
        UPDATE: `${API_URL}/storekeeper/update`,
        AUTHORIZE: `${API_URL}/storekeeper/authorize`
    }
    async create(item: Storekeeper): Promise<Storekeeper> {
        const response = await axios.post<Storekeeper>(StorekeeperService.STOREKEEPER_URLS.CREATE, item);
        return response.data;
    }
    async delete(itemId: number): Promise<void> {
        await axios.delete(StorekeeperService.STOREKEEPER_URLS.DELETE(itemId));
    }
    async update(item: Storekeeper): Promise<Storekeeper> {
        const response = await axios.put<Storekeeper>(StorekeeperService.STOREKEEPER_URLS.UPDATE, item);
        return response.data;
    }
    async getAll(): Promise<Storekeeper[]> {
        const response = await axios.get<Storekeeper[]>(StorekeeperService.STOREKEEPER_URLS.GET_ALL);
        return response.data;
    }
    async getById(id: number): Promise<Storekeeper | null> {
        const response = await axios.get<Storekeeper | null>(StorekeeperService.STOREKEEPER_URLS.GET_BY_ID(id));
        return response.data;
    }
    async authorize(username: string, passwordHash: string): Promise<Storekeeper | null> {
        const response = await axios.post<Storekeeper | null>(StorekeeperService.STOREKEEPER_URLS.AUTHORIZE, {username: username, password_hash: passwordHash});
        return response.data;
    }
}