import { useEffect, useState } from "react";
import { FeedSupplyService } from "../../api/services/FeedSupplyService";
import { FeedSupplyVM } from "../../viewModels/FeedSupplyVM";
import styles from "../../assets/css/FeedSuppliesList.module.css";
import { useTranslation } from "react-i18next";

interface IFeedSuppliesList{
    feedId: number;
}


export const FeedSuppliesList: React.FC<IFeedSuppliesList> =({feedId}) =>{
    const feedSupplyService = new FeedSupplyService();
    const [feedSuppliesVMList, setFeedSuppliesVMList] = useState<FeedSupplyVM[]>([]);
    
    const {t} = useTranslation();

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
                        <h2 className={styles.supplyId}>{t("feedSuppliesList.supplyHeader")} #{feedSupply.id}</h2>
                    </div>
                    <div className={styles.cardBody}>
                            <p><strong>{t("feedSuppliesList.performerHeader")}:</strong> {feedSupply.storekeeperName !== null ? `${feedSupply.storekeeperName} ${feedSupply.storekeeperSurname}` : `${t("feedSuppliesList.noDataHeader")}`}</p>
                            <p><strong>{t("feedSuppliesList.feedAmountHeader")}:</strong> {feedSupply.amount} {t("feedSuppliesList.feedAmountUnitsHeader")}</p>
                            <p><strong>{t("feedSuppliesList.dateHeader")}:</strong> {feedSupply.date}</p>
                    </div>
                </div>
            ))) : (
                <p>{t("feedSuppliesList.notFoundHeader")}</p>
            )}
        </div>
    )
}