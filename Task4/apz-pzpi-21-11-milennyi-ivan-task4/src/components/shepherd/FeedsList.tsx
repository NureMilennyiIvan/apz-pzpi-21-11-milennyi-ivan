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
        <div className={styles.list}>
            {feedsVMList.length > 0 ? (
                feedsVMList.map((feed) => (
                    <div key={feed.id} className={styles.card}>
                        <div className={styles.cardHeader}>
                            <h2 className={styles.feedName}>{feed.name}</h2>
                        </div>
                        <div className={styles.cardBody}>
                            <p><strong>Кількість:</strong> {feed.amount} кг</p>
                            <p><strong>Калорії:</strong> {feed.calories} ккал</p>
                            <p><strong>Жири:</strong> {feed.fat} г</p>
                            <p><strong>Білки:</strong> {feed.protein} г</p>
                            <p><strong>Вуглеводи:</strong> {feed.carbohydrates} г</p>
                            {feed.breedName ? (
                                <p><strong>Порода:</strong> {feed.breedName}</p>
                            ) : (
                                <p><strong>Порода:</strong> Не призначена</p>
                            )}
                            <p><strong>Кількість овець:</strong> {feed.sheepCount}</p>
                        </div>
                    </div>
                ))
            ) : (
                <p>Немає даних про корми</p>
            )}
        </div>
    );
}
