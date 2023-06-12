require 'date'
require 'ffi'
require 'json'
require 'net/http'
require 'rbconfig'

module Shadow

    class LoadShadowOptions
        attr_reader :shadow_url
        def initialize(shadow_url:)
            @shadow_url = shadow_url
        end
    end

    class PredictionOptions
        attr_reader :threshold

        attr_reader :compute_feature_contributions

        def initialize(compute_feature_contributions:, threshold: nil)
            @threshold = threshold
            @compute_feature_contributions = compute_feature_contributions
        end

        def to_json(*args)
            {'threshold' => @threshold, 'compute_feature_contributions' => @compute_feature_contributions}.to_json(*args)
        end
    end

end