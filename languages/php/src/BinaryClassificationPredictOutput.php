<?php

declare(strict_types=1);

namespace tangram\tangram;

final class BinaryClassificationPredictOutput extends PredictOutput
{
    /**
     * The name of the predicted class.
     */
    public string $class_name;
    /**
     * This is the probability the model assigned to the predicted class.
     */
    public float $probability;
    /**
     *  If computing feature contributions was enabled in the predict options, this value will explain the model's output, showing how much each feature contributed to the output.
     */
    public ?FeatureContributions $feature_contributions;
    /**
     * Create a new PredictOptions instance
     * @param string $class_name
     * @param float $probability
     * @param ?FeatureContributions $feature_contributions
     * @return void
     */
    public function __construct(string $class_name, float $probability, ?FeatureContributions $feature_contributions = null)
    {
        $this->class_name = $class_name;
        $this->probability = $probability;
        $this->feature_contributions = $feature_contributions;
    }

    /**
     * Serialize to JSON
     * @return string JSON representation
     */
    public function to_json()
    {
        return json_encode($this);
    }
}
