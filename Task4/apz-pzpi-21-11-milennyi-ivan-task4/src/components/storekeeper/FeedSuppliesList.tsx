import { useEffect, useState } from "react";
import { FeedSupplyService } from "../../api/services/FeedSupplyService";
import { FeedSupplyVM } from "../../viewModels/FeedSupplyVM";
import styles from "../../assets/css/FeedSuppliesList.module.css";

interface IFeedSuppliesList{
    feedId: number;
}


export const FeedSuppliesList: React.FC<IFeedSuppliesList> =({feedId}) =>{
    const feedSupplyService = new FeedSupplyService();
    const [feedSuppliesVMList, setFeedSuppliesVMList] = useState<FeedSupplyVM[]>([]);
    
    useEffect(() => {
        const fetchFeedSupplies = async () =>{
            try{
                const data = await feedSupplyService.getAllVMsByFeedId(feedId);
                setFeedSuppliesVMList(data);
            }
            catch (error){
                alert(error);
                setFeedSuppliesVMList([]);
            }
        }
        fetchFeedSupplies();
    }, []);
    return (
        <div>
            {feedSuppliesVMList.length > 0 ? (feedSuppliesVMList.map((feedSupply) => (
                <div key={feedSupply.id} className={styles.container}>
                    <div>
                        <h4>{feedSupply.id}</h4>
                    </div>
                    <div>
                        <h4>{feedSupply.date}</h4>
                    </div>
                    <div>
                        <h4>{feedSupply.amount}</h4>
                    </div>
                    <div>
                        <h4>{feedSupply.storekeeperName}</h4>
                    </div>
                    <div>
                        <h4>{feedSupply.storekeeperSurname}</h4>
                    </div>
                </div>
            ))) : (
                <p></p>
            )}
        </div>
    )
}