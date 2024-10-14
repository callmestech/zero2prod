FROM prom/prometheus:latest

# Copy your Prometheus configuration file into the container
COPY prometheus.yaml /etc/prometheus/

# Expose the Prometheus web UI port
EXPOSE 9090

# Start Prometheus
CMD [ "--config.file=/etc/prometheus/prometheus.yaml" ]
