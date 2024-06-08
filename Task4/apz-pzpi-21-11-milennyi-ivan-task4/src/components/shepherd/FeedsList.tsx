import { useEffect, useState } from "react";
import { FeedService } from "../../api/services/FeedService";
import { FeedVM } from "../../viewModels/FeedVM";
import styles from '../../assets/css/FeedsList.module.css';

export const FeedsList = () => {
    const feedService = new FeedService();
    const [feedsVMList, setFeedsVMList] = useState<FeedVM[]>([]);
    
    useEffect(() => {
        const fetchFeeds = async () => {
            try {
                const data = await feedService.getAllVMs();
                setFeedsVMList(data);
            } catch (error) {
                alert(error);
                setFeedsVMList([]);
            }
        }
        fetchFeeds();
    }, []);
    
    return (
        <div>
            {feedsVMList.length > 0 ? (feedsVMList.map((feed) => (
                <div key={feed.id} className={styles.container}>
                    <div>
                        <h4>{feed.name}</h4>
                    </div>
                    <div>
                        <h4>{feed.amount}</h4>
                    </div>
                    <div>
                        <h4>{feed.calories}</h4>
                    </div>
                    <div>
                        <h4>{feed.fat}</h4>
                    </div>
                    <div>
                        <h4>{feed.protein}</h4>
                    </div>
                    <div>
                        <h4>{feed.carbohydrates}</h4>
                    </div>
                    <div>
                        <h4>{feed.breedName}</h4>
                    </div>
                    <div>
                        <h4>{feed.sheepCount}</h4>
                    </div>
                </div>
            ))) : (
                <p></p>
            )}
        </div>
    );
}
