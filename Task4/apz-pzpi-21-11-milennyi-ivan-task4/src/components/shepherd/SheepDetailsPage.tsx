import { useEffect, useState } from "react";
import { SheepService } from "../../api/services/SheepService";
import { useTranslation } from "react-i18next";
import { SheepDetailsVM } from "../../viewModels/extraViewModels/SheepDetailsVM";
import { FeedingLog } from "../../models/FeedingLog";
import { FeedingLogService } from "../../api/services/FeedingLogService";
import { ShearingLog } from "../../models/ShearingLog";
import { ShearingLogService } from "../../api/services/ShearingLogService";
import styles from '../../assets/css/SheepDetailsPage.module.css';

interface ISheepDetailsPage {
    shepherdId: number;
    sheepId: number;
}

export const SheepDetailsPage: React.FC<ISheepDetailsPage> = ({shepherdId, sheepId}) => {
    const sheepService = new SheepService();
    const feedingLogService = new FeedingLogService();
    const shearingLogService = new ShearingLogService();
    const [sheepDetails, setSheepDetails] = useState<SheepDetailsVM | null>();
    const [trigger, setTrigger] = useState<boolean>(true);
    const [woolAmount, setWoolAmount] = useState<string>('');
    const [errorMessage, setErrorMessage] = useState<string>('');
    const {t} = useTranslation();

    useEffect(() => {
        const fetchSheepDetails = async () => {
            try {
                const data = await sheepService.getDetailsById(sheepId);
                setErrorMessage('');
                setWoolAmount('');
                setSheepDetails(data);
            } catch (error) {
                alert(error);
                setSheepDetails(null);
            }
        }
        fetchSheepDetails();
    }, [trigger]);

    const createFeedingLog = async (details: SheepDetailsVM) => {
        try {
            const feedingLog = new FeedingLog(null, sheepId, shepherdId, new Date().getTime(), details.feedId, Math.floor(details.requiredFeedAmount * 1000));
            await feedingLogService.create(feedingLog);
            setTrigger(!trigger);
        } catch (error) {
            console.log(error);
            setErrorMessage(t("в"));
        }
    }

    const createShearingLog = async () => {
        if (woolAmount.length == 0 || !(/^(0|[1-9]\d*)$/.test(woolAmount))) {
            setErrorMessage(t("в"));
            return;
        }
        setErrorMessage('');
        try {
            const shearingLog = new ShearingLog(null, sheepId, shepherdId, new Date().getTime(), Math.floor(parseInt(woolAmount) * 1000));
            await shearingLogService.create(shearingLog);
            setTrigger(!trigger);
        } catch (error) {
            console.log(error);
            setErrorMessage(t("в"));
        }
    };

    return (
        <div className={styles.container}>
            {sheepDetails ? (
                <div>
                    <div className={styles.cardHeader}>
                        <h2 className={styles.sheepId}>Вівця #{sheepDetails.id}</h2>
                    </div>
                    <div className={styles.details}>
                        <div className={styles.infoGroup}>
                        <div className={styles.infoItem}>
                            <p><strong>Порода:</strong> {sheepDetails.breed}</p>
                        </div>
                        <div className={styles.infoItem}>
                            <p><strong>Вік:</strong> {sheepDetails.age} днів</p>
                        </div>
                        <div className={styles.infoItem}>
                            <p><strong>Стать:</strong> {sheepDetails.sex ? 'Самець' : 'Самка'}</p>
                        </div>
                        <div className={styles.infoItem}>
                            <div className={styles.temperature}>
                                <p><strong>Температура:</strong> {sheepDetails.temperature ? `${sheepDetails.temperature} °C` : 'Немає даних'}</p>
                            </div>
                            {sheepDetails.temperature !== null ? (
                                <p>{sheepDetails.temperature >= 36 && sheepDetails.temperature <= 37.5 ? 'Температура в нормі' : 'Температура ненормальна'}</p>
                            ) : (
                                <p>Сканер температури відсутній</p>
                            )}
                        </div>
                        <div className={styles.infoItem}>
                            <p><strong>Вага:</strong> {sheepDetails.weight} кг</p>
                        </div>
                        <div className={styles.infoItem}>
                            <p><strong>Корм:</strong> {sheepDetails.feedName}</p>
                        </div>
                        <div className={styles.infoItem}>
                            <p><strong>Доступно корму:</strong> {sheepDetails.availableFeedAmount} кг</p>
                        </div>
                    </div>
                    <div className={styles.statusGroup}>
                        <div className={styles.infoItem}>
                            <p><strong>Кількість корму для годування:</strong> {sheepDetails.requiredFeedAmount} кг</p>
                        </div>
                        <div className={styles.infoItem}>
                            <div className={styles.feeding}>
                                <p><strong>Останнє годування:</strong> {sheepDetails.lastFeedingDate ? sheepDetails.lastFeedingDate : "Немає даних"}</p>                       
                            </div>
                            {sheepDetails.isFeed ? (
                                <div>
                                    {sheepDetails.requiredFeedAmount <= sheepDetails.availableFeedAmount ? ( 
                                        <div className={styles.form}>
                                            <button className={styles.button} onClick={() => createFeedingLog(sheepDetails)}>Нагодувати вівцю</button>

                                        </div>
                                    ) : (                                
                                        <p>Недостатньо корму</p>
                                     )}
                                </div>

                            ) : (
                                <p>Годування не потрібно</p>
                            )}
                        </div>
                        <div className={styles.infoItem}>
                            <div className={styles.shearing}>
                                <p><strong>Остання стрижка:</strong> {sheepDetails.lastShearingDate ? sheepDetails.lastShearingDate : "Немає даних"}</p>
                            </div>
                            {sheepDetails.isShear ? (
                                <div>
                                    <input className={styles.input} type="text" value={woolAmount} onChange={(e) => setWoolAmount(e.target.value)} placeholder="Введіть зістриженої кількість вовни" />

                                    <button className={styles.button} onClick={() => createShearingLog()}>Постригти вівцю</button>
                                </div>
                            ) : (
                                <p>Стрижка не потрібна</p>
                            )}
                        </div>
                        <div className={styles.errorItem}>
                            {errorMessage && <span className={styles.error}>{errorMessage}</span>}
                        </div>
                    </div>

                </div>
                <div className={styles.infoItem}>
                    <p><strong>Інформація про породу:</strong> {sheepDetails.breedInfo}</p>
                </div>
            </div>
            ) : (
                <p>404 NotFound</p>
            )}
        </div>
    )
}
