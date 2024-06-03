import { FeedSupply } from "../../models/FeedSupply";
import { FeedSupplyVM } from "../../viewModels/FeedSupplyVM";
import { IFeedSupplyService } from "../interfaces/IFeedSupplyService";

export class FeedSupplyService implements IFeedSupplyService{
    getAllVMs(): Promise<FeedSupplyVM[]> {
        throw new Error("Method not implemented.");
    }
    create(item: FeedSupply): Promise<FeedSupply> {
        throw new Error("Method not implemented.");
    }
    delete(itemId: number): Promise<void> {
        throw new Error("Method not implemented.");
    }
    update(item: FeedSupply): Promise<FeedSupply> {
        throw new Error("Method not implemented.");
    }
    getAll(): Promise<FeedSupply[]> {
        throw new Error("Method not implemented.");
    }
    getById(id: number): Promise<FeedSupply | null> {
        throw new Error("Method not implemented.");
    }
}