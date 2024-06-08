import { useState } from "react";
import { IUserProps } from "../properties/IUserProps";
import { useNavigate, useParams } from "react-router-dom";
import { useTranslation } from "react-i18next";
import { useEffectUser } from "../../utils/helpers";
import { FeedSupply } from "../../models/FeedSupply";
import { FeedSupplyService } from "../../api/services/FeedSupplyService";
import styles from '../../assets/css/CreateFeedSupplyForm.module.css';

export const CreateFeedSupplyForm: React.FC<IUserProps> = ({user}) => {
    const [amount, setAmount] = useState<string>('');
    const [errorAmount, setErrorAmount] = useState<string>('');
    const {feedId} = useParams();
    const feedSupplyService = new FeedSupplyService();

    const navigate = useNavigate();
    const {t} = useTranslation();
    useEffectUser(user, navigate);

    const createFeedSupply = async () => {
        if (amount.length == 0 || !(/^(0|[1-9]\d*)$/.test(amount))) {
            setErrorAmount("gjhgj");
            return;
        }
        setErrorAmount('');
        try {
            const feedSupply = new FeedSupply(null, user.Id!, parseInt(amount) * 1000, new Date().getTime(), parseInt(feedId!));
            await feedSupplyService.create(feedSupply);
            navigate(-1);
        } catch (error) {
            console.log(error);
            setErrorAmount(t(""));
        }
    };

    return (
        <div className={styles.container}>
            <div className={styles.form}>
                <div>
                    <div className={styles.label}>
                        <label >{t("createFeedSupplyForm.amountLabel")}</label>
                    </div>
                    <input className={styles.input} type="text" value={amount} onChange={(e) => setAmount(e.target.value)} placeholder="Enter amount" />
                    <div className={styles.error}>
                        {errorAmount && <span>{errorAmount}</span>}
                    </div>
                </div>
                <div>
                    <button className={styles.button} onClick={createFeedSupply}>{t("createFeedSupplyForm.submitButton")}</button>
                </div>
            </div>
        </div>
    )
}
