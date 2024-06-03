import axios from "axios";
import { FeedSupply } from "../../models/FeedSupply";
import { API_URL } from "../../utils/config";
import { FeedSupplyVM } from "../../viewModels/FeedSupplyVM";
import { IFeedSupplyService } from "../interfaces/IFeedSupplyService";

export class FeedSupplyService implements IFeedSupplyService{
    private static FEED_SUPPLY_URLS = {
        GET_ALL: `${API_URL}/feed-supply`,
        GET_BY_ID: (id: number) => `${API_URL}/feed-supply/${id}`,
        CREATE: `${API_URL}/feed-supply/create`,
        DELETE: (id: number) => `${API_URL}/feed-supply/delete/${id}`,
        GET_ALL_VMS: `${API_URL}/feed-supply-vms`
    }
    async create(item: FeedSupply): Promise<FeedSupply> {
        const response = await axios.post<FeedSupply>(FeedSupplyService.FEED_SUPPLY_URLS.CREATE, item);
        return response.data;
    }
    async delete(itemId: number): Promise<void> {
        await axios.delete(FeedSupplyService.FEED_SUPPLY_URLS.DELETE(itemId));
    }
    async update(_item: FeedSupply): Promise<FeedSupply> {
        throw new Error("Method is forbidden.");
    }
    async getAll(): Promise<FeedSupply[]> {
        const response = await axios.get<FeedSupply[]>(FeedSupplyService.FEED_SUPPLY_URLS.GET_ALL);
        return response.data;
    }
    async getById(id: number): Promise<FeedSupply | null> {
        const response = await axios.get<FeedSupply | null>(FeedSupplyService.FEED_SUPPLY_URLS.GET_BY_ID(id));
        return response.data;
    }
    async getAllVMs(): Promise<FeedSupplyVM[]> {
        const response = await axios.get<FeedSupplyVM[]>(FeedSupplyService.FEED_SUPPLY_URLS.GET_ALL_VMS);
        return response.data;
    }
}