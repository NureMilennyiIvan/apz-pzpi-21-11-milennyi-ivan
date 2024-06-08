import { useEffect, useState } from "react";
import { FeedService } from "../../api/services/FeedService";
import { FeedVM } from "../../viewModels/FeedVM";
import { useNavigate } from "react-router-dom";
import { useTranslation } from "react-i18next";
import styles from "../../assets/css/StorekeeperMainPage.module.css";
export const StorekeeperMainPage = () =>{
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
        <div className={styles.content}>
            {feedsVMList.length > 0 ? (feedsVMList.map((feed) => (
            <div key={feed.id} className={styles.container}>
                <div onClick={() => navigate("feed/" + feed.id)}>
                    <div>
                        <h4>{feed.name}</h4>
                    </div>
                    <div>
                        <h4>{feed.amount}</h4>
                    </div>
                    <div>
                        <h4>{feed.breedName}</h4>
                    </div>
                    <div>
                        <h4>{feed.sheepCount}</h4>
                    </div>
                </div>
                <div>
                    <button onClick={() => navigate("create/feed-supply/" + feed.id)}></button>
                </div>
            </div>
          ))) : (
            <p></p>
          )}
        </div>
    );
}