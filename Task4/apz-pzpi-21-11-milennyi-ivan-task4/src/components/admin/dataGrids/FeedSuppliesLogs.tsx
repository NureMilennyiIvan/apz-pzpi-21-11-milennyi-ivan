import { useEffect, useState } from "react";
import styles from "../../../assets/css/GridComponent.module.css"
import { useTranslation } from "react-i18next";
import { FeedSupplyService } from "../../../api/services/FeedSupplyService";
import { FeedSupply } from "../../../models/FeedSupply";

export const FeedSuppliesGrid = () =>{
    const feedSupplyService = new FeedSupplyService();
    const [selectedFeedSupply, setSelectedFeedSupply] = useState<FeedSupply | null>(null);
    const [feedSupplies, setFeedSupplies] = useState<FeedSupply[]>([]);
    const [trigger, setTrigger] = useState<boolean>(true);

    const {t} = useTranslation();
    
    useEffect(() => {
        const fetchFeedSupplies= async () => {
          try {
            const data = await feedSupplyService.getAll();
            setFeedSupplies(data);
          } catch (error) {
            alert(error);
            setFeedSupplies([]);
          }
        };
        fetchFeedSupplies();
    }, [trigger]);

    const handleRowSelection = (feedSupply: FeedSupply) => {
        setSelectedFeedSupply(feedSupply);
    };

    const deleteFeedSupply = async (id: number) => {
        try{
            await feedSupplyService.delete(id);
            setSelectedFeedSupply(null);
            setTrigger(!trigger);
        }
        catch(error){
            console.log(error);
            alert("Error");
        }
    }
  
    return (
        <div className={styles.container}>
            <table className={styles.table}>
                <thead>
                    <tr className={styles.tr}>
                        <th className={styles.th}>{t("gridBase.chooseHeader")}</th>
                        <th className={styles.th}>{t("gridBase.idHeader")}</th>
                        <th className={styles.th}>{t("feedSuppliesGrid.amountHeader")}</th>
                        <th className={styles.th}>{t("feedSuppliesGrid.feedIdHeader")}</th>
                        <th className={styles.th}>{t("feedSuppliesGrid.storekeeperIdHeader")}</th>
                        <th className={styles.th}>{t("feedSuppliesGrid.timestampHeader")}</th>
                    </tr>
                </thead>
                <tbody>
                    {feedSupplies.length != 0 ? (
                        feedSupplies.map((feedSupply) => (
                            <tr key={feedSupply.id} 
                                className={`${styles.tr} ${selectedFeedSupply?.id === feedSupply.id ? styles.selected : ''}`}>
                                <td className={styles.td}>
                                    <input
                                        type="radio"
                                        name="selectedUser"
                                        checked={selectedFeedSupply?.id === feedSupply.id}
                                        onChange={() => handleRowSelection(feedSupply)}
                                    />
                                </td>
                                <td className={styles.td}>{feedSupply.id}</td>
                                <td className={styles.td}>{feedSupply.amount}</td>
                                <td className={styles.td}>{feedSupply.feed_id}</td>
                                <td className={styles.td}>{feedSupply.storekeeper_id ? feedSupply.storekeeper_id : "null"}</td>
                                <td className={styles.td}>{feedSupply.timestamp}</td>
                            </tr>
                        ))
                    ) : (
                        <div className={styles.error}>{t("feedSuppliesGrid.notFoundHeader")}</div>
                    )}
                </tbody>
            </table>
            <div className={styles.actionButtonsContainer}>
                <button 
                    className={`${styles.actionButton} ${selectedFeedSupply?.id ? '' : styles.disabledButton}`}
                    onClick={() => selectedFeedSupply ? deleteFeedSupply(selectedFeedSupply.id!) : undefined}
                    style={{ cursor: selectedFeedSupply ? 'pointer' : 'default' }}>
                    {t("gridBase.deleteButtonText")}
                </button>
            </div>
        </div>
    );
}