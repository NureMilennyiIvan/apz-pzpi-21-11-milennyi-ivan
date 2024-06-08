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
        <div className={styles.list}>
            {feedSuppliesVMList.length > 0 ? (feedSuppliesVMList.map((feedSupply) => (
                <div key={feedSupply.id} className={styles.card}>
                    <div className={styles.cardHeader}>
                        <h2 className={styles.supplyId}>Постачання #{feedSupply.id}</h2>
                    </div>
                    <div className={styles.cardBody}>
                            <p><strong>Виконавець:</strong> {feedSupply.storekeeperName !== null ? `${feedSupply.storekeeperName} ${feedSupply.storekeeperSurname}` : 'Немає даних'}</p>
                            <p><strong>Додано корму:</strong> {feedSupply.amount} кг</p>
                            <p><strong>Дата:</strong> {feedSupply.date}</p>
                    </div>
                </div>
            ))) : (
                <p>Немає інформації про постачання</p>
            )}
        </div>
    )
}