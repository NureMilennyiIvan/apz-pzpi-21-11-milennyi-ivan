import { useEffect, useState } from "react";
import { useNavigate, useParams } from "react-router-dom";
import { useTranslation } from "react-i18next";
import { handleElementChange, useEffectUser } from "../../utils/helpers";
import styles from "../../../assets/css/AdminBaseForm.module.css";
import { ShepherdService } from "../../api/services/ShepherdService";
import { StorekeeperService } from "../../api/services/StorekeeperService";
import { SheepService } from "../../api/services/SheepService";
import { FeedService } from "../../api/services/FeedService";
import { BreedService } from "../../api/services/BreedService";
import { TemperatureScannerService } from "../../api/services/TemperatureScannerService";
import { IUserProps } from "../properties/IUserProps";

interface FormProps extends IUserProps {
  entityType: "Shepherd" | "Storekeeper" | "Sheep" | "Feed" | "Breed" | "TemperatureScanner";
}

export const AdminBaseForm: React.FC<FormProps> = ({ user, entityType }) => {
  const { entityId } = useParams();
  const [entityData, setEntityData] = useState<any>(null);
  const [formData, setFormData] = useState<any>({});
  const [errors, setErrors] = useState<any>({});
  const [trigger, setTrigger] = useState(true);
  const navigate = useNavigate();
  const { t } = useTranslation();
  useEffectUser(user, navigate);

  const services = {
    Shepherd: new ShepherdService(),
    Storekeeper: new StorekeeperService(),
    Sheep: new SheepService(),
    Feed: new FeedService(),
    Breed: new BreedService(),
    TemperatureScanner: new TemperatureScannerService(),
  };

  useEffect(() => {
    const fetchData = async () => {
      if (entityId) {
        try {
          const data = await services[entityType].getById(parseInt(entityId as string));
          setEntityData(data);
          setFormData({ ...data });
        } catch (error) {
          alert(error);
          setEntityData(null);
        }
      }
    };
    fetchData();
  }, [trigger, entityId]);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    const validationErrors = validateForm(formData);
    if (Object.keys(validationErrors).length > 0) {
      setErrors(validationErrors);
      return;
    }
    setErrors({});
    try {
      if (entityId) {
        await services[entityType].update(formData);
      } else {
        await services[entityType].create(formData);
      }
      navigate(-1);
    } catch (error) {
      console.log(error);
      setErrors({ submit: "Error saving data" });
    }
  };

  const validateForm = (data: any) => {
    const errors: any = {};
    // Add validation logic for different entity types
    return errors;
  };

  const renderFormFields = () => {
    switch (entityType) {
      case "Shepherd":
      case "Storekeeper":
        return (
          <>
            <div className={styles.formGroup}>
              <label className={styles.label}>Username</label>
              <input
                className={styles.input}
                type="text"
                value={formData.username || ""}
                onChange={(e) => handleElementChange(e, (value) => setFormData({ ...formData, username: value }))}
              />
              {errors.username && <span className={styles.error}>{errors.username}</span>}
            </div>
            <div className={styles.formGroup}>
              <label className={styles.label}>Password</label>
              <input
                className={styles.input}
                type="password"
                value={formData.password || ""}
                onChange={(e) => handleElementChange(e, (value) => setFormData({ ...formData, password: value }))}
              />
              {errors.password && <span className={styles.error}>{errors.password}</span>}
            </div>
            <div className={styles.formGroup}>
              <label className={styles.label}>Name</label>
              <input
                className={styles.input}
                type="text"
                value={formData.name || ""}
                onChange={(e) => handleElementChange(e, (value) => setFormData({ ...formData, name: value }))}
              />
              {errors.name && <span className={styles.error}>{errors.name}</span>}
            </div>
            <div className={styles.formGroup}>
              <label className={styles.label}>Surname</label>
              <input
                className={styles.input}
                type="text"
                value={formData.surname || ""}
                onChange={(e) => handleElementChange(e, (value) => setFormData({ ...formData, surname: value }))}
              />
              {errors.surname && <span className={styles.error}>{errors.surname}</span>}
            </div>
          </>
        );
      case "Sheep":
        return (
          <>
            <div className={styles.formGroup}>
              <label className={styles.label}>Birth Date</label>
              <input
                className={styles.input}
                type="date"
                value={formData.birth_date || ""}
                onChange={(e) => handleElementChange(e, (value) => setFormData({ ...formData, birth_date: value }))}
              />
              {errors.birth_date && <span className={styles.error}>{errors.birth_date}</span>}
            </div>
            <div className={styles.formGroup}>
              <label className={styles.label}>Breed ID</label>
              <input
                className={styles.input}
                type="number"
                value={formData.breed_id || ""}
                onChange={(e) => handleElementChange(e, (value) => setFormData({ ...formData, breed_id: value }))}
              />
              {errors.breed_id && <span className={styles.error}>{errors.breed_id}</span>}
            </div>
            <div className={styles.formGroup}>
              <label className={styles.label}>Weight</label>
              <input
                className={styles.input}
                type="number"
                value={formData.weight || ""}
                onChange={(e) => handleElementChange(e, (value) => setFormData({ ...formData, weight: value }))}
              />
              {errors.weight && <span className={styles.error}>{errors.weight}</span>}
            </div>
            <div className={styles.formGroup}>
              <label className={styles.label}>Sex</label>
              <select
                className={styles.input}
                value={formData.sex || ""}
                onChange={(e) => handleElementChange(e, (value) => setFormData({ ...formData, sex: value }))}
              >
                <option value="true">Male</option>
                <option value="false">Female</option>
              </select>
              {errors.sex && <span className={styles.error}>{errors.sex}</span>}
            </div>
            <div className={styles.formGroup}>
              <label className={styles.label}>Temperature Scanner ID</label>
              <input
                className={styles.input}
                type="number"
                value={formData.temperature_scanner_id || ""}
                onChange={(e) => handleElementChange(e, (value) => setFormData({ ...formData, temperature_scanner_id: value }))}
              />
              {errors.temperature_scanner_id && <span className={styles.error}>{errors.temperature_scanner_id}</span>}
            </div>
            <div className={styles.formGroup}>
              <label className={styles.label}>Shepherd ID</label>
              <input
                className={styles.input}
                type="number"
                value={formData.shepherd_id || ""}
                onChange={(e) => handleElementChange(e, (value) => setFormData({ ...formData, shepherd_id: value }))}
              />
              {errors.shepherd_id && <span className={styles.error}>{errors.shepherd_id}</span>}
            </div>
          </>
        );
      case "Feed":
        return (
          <>
            <div className={styles.formGroup}>
              <label className={styles.label}>Name</label>
              <input
                className={styles.input}
                type="text"
                value={formData.name || ""}
                onChange={(e) => handleElementChange(e, (value) => setFormData({ ...formData, name: value }))}
              />
              {errors.name && <span className={styles.error}>{errors.name}</span>}
            </div>
            <div className={styles.formGroup}>
              <label className={styles.label}>Amount</label>
              <input
                className={styles.input}
                type="number"
                value={formData.amount || ""}
                onChange={(e) => handleElementChange(e, (value) => setFormData({ ...formData, amount: value }))}
              />
              {errors.amount && <span className={styles.error}>{errors.amount}</span>}
            </div>
            <div className={styles.formGroup}>
              <label className={styles.label}>Calories</label>
              <input
                className={styles.input}
                type="number"
                value={formData.calories || ""}
                onChange={(e) => handleElementChange(e, (value) => setFormData({ ...formData, calories: value }))}
              />
              {errors.calories && <span className={styles.error}>{errors.calories}</span>}
            </div>
            <div className={styles.formGroup}>
              <label className={styles.label}>Fat</label>
              <input
                className={styles.input}
                type="number"
                value={formData.fat || ""}
                onChange={(e) => handleElementChange(e, (value) => setFormData({ ...formData, fat: value }))}
              />
              {errors.fat && <span className={styles.error}>{errors.fat}</span>}
            </div>
            <div className={styles.formGroup}>
              <label className={styles.label}>Protein</label>
              <input
                className={styles.input}
                type="number"
                value={formData.protein || ""}
                onChange={(e) => handleElementChange(e, (value) => setFormData({ ...formData, protein: value }))}
              />
              {errors.protein && <span className={styles.error}>{errors.protein}</span>}
            </div>
            <div className={styles.formGroup}>
              <label className={styles.label}>Carbohydrates</label>
              <input
                className={styles.input}
                type="number"
                value={formData.carbohydrates || ""}
                onChange={(e) => handleElementChange(e, (value) => setFormData({ ...formData, carbohydrates: value }))}
              />
              {errors.carbohydrates && <span className={styles.error}>{errors.carbohydrates}</span>}
            </div>
          </>
        );
      case "Breed":
        return (
          <>
            <div className={styles.formGroup}>
              <label className={styles.label}>Name</label>
              <input
                className={styles.input}
                type="text"
                value={formData.name || ""}
                onChange={(e) => handleElementChange(e, (value) => setFormData({ ...formData, name: value }))}
              />
              {errors.name && <span className={styles.error}>{errors.name}</span>}
            </div>
            <div className={styles.formGroup}>
              <label className={styles.label}>Feed ID</label>
              <input
                className={styles.input}
                type="number"
                value={formData.feed_id || ""}
                onChange={(e) => handleElementChange(e, (value) => setFormData({ ...formData, feed_id: value }))}
              />
              {errors.feed_id && <span className={styles.error}>{errors.feed_id}</span>}
            </div>
            <div className={styles.formGroup}>
              <label className={styles.label}>Info</label>
              <textarea
                className={styles.input}
                value={formData.info || ""}
                onChange={(e) => handleElementChange(e, (value) => setFormData({ ...formData, info: value }))}
              />
              {errors.info && <span className={styles.error}>{errors.info}</span>}
            </div>
          </>
        );
      case "TemperatureScanner":
        return (
          <>
            <div className={styles.formGroup}>
              <label className={styles.label}>Temperature</label>
              <input
                className={styles.input}
                type="number"
                value={formData.temperature || ""}
                onChange={(e) => handleElementChange(e, (value) => setFormData({ ...formData, temperature: value }))}
              />
              {errors.temperature && <span className={styles.error}>{errors.temperature}</span>}
            </div>
            <div className={styles.formGroup}>
              <label className={styles.label}>Password</label>
              <input
                className={styles.input}
                type="password"
                value={formData.password || ""}
                onChange={(e) => handleElementChange(e, (value) => setFormData({ ...formData, password: value }))}
              />
              {errors.password && <span className={styles.error}>{errors.password}</span>}
            </div>
          </>
        );
      default:
        return null;
    }
  };

  return (
    <div className={styles.container}>
      <div className={styles.form}>
        <form onSubmit={handleSubmit}>
          {renderFormFields()}
          {errors.submit && <span className={styles.error}>{errors.submit}</span>}
          <div className={styles.actionButtonsContainer}>
            <button type="submit" className={styles.actionButton}>
              {entityId ? t("editButtonText") : t("createButtonText")}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
};

