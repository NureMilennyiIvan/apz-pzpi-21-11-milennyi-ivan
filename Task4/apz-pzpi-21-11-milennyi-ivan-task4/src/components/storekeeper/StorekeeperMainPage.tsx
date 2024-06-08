import { useEffect, useState } from "react";
import { FeedService } from "../../api/services/FeedService";
import { FeedVM } from "../../viewModels/FeedVM";
import { useNavigate } from "react-router-dom";
import { useTranslation } from "react-i18next";
import styles from "../../assets/css/StorekeeperMainPage.module.css";
import { IUserProps } from "../properties/IUserProps";

export const StorekeeperMainPage: React.FC<IUserProps> = () =>{
    const feedService = new FeedService();
    const [feedsVMList, setFeedsVMList] = useState<FeedVM[]>([]);
    const navigate = useNavigate();
    const {t} = useTranslation();
    
    useEffect(() => {
        const fetchFeeds = async () =>{
            try{
                const data = await feedService.getAllVMs();
                setFeedsVMList(data);
            }
            catch (error){
                alert(error);
                setFeedsVMList([]);
            }
        }
        fetchFeeds();
    }, []);
    return (
        <div className={styles.container}>
        <div className={styles.list}>
            {feedsVMList.length > 0 ? (feedsVMList.map((feed) => (
            <div key={feed.id} className={styles.card}>
                <div onClick={() => navigate("feed/" + feed.id)}>
                    <div className={styles.cardHeader}>
                        <h2 className={styles.feedName}>{feed.name}</h2>
                    </div>
                    <div className={styles.cardBody}>
                        <p><strong>Порода:</strong> {feed.breedName}</p>
                        <p><strong>Кількість:</strong> {feed.amount}</p>
                        <p><strong>Кількість овець:</strong> {feed.sheepCount}</p>
                    </div>
                    <button className={styles.button} onClick={(e) => { e.stopPropagation(); navigate("create/feed-supply/" + feed.id)}}>Додати корм</button>
                </div>
            </div>
          )))  : (
            <p>Немає даних про корм</p>
        )}
        </div>
        </div> 

    );
}