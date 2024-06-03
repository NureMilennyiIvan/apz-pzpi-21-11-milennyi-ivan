import { Feed } from "../../models/Feed";
import { FeedVM } from "../../viewModels/FeedVM";
import { IFeedService } from "../interfaces/IFeedServiceService";

export class FeedService implements IFeedService {
    getAllVMs(): Promise<FeedVM[]> {
        throw new Error("Method not implemented.");
    }
    create(item: Feed): Promise<Feed> {
        throw new Error("Method not implemented.");
    }
    delete(itemId: number): Promise<void> {
        throw new Error("Method not implemented.");
    }
    update(item: Feed): Promise<Feed> {
        throw new Error("Method not implemented.");
    }
    getAll(): Promise<Feed[]> {
        throw new Error("Method not implemented.");
    }
    getById(id: number): Promise<Feed | null> {
        throw new Error("Method not implemented.");
    }
}