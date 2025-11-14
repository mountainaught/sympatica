"""
models.py - The data models. There are three models: Patient, Session and Reading.

Patient has a one-to-many relationship with Session (one patient can have many sessions), and Session has the same with
Reading (one session/many readings).
"""
import uuid
from django.db import models
from django.utils import timezone


class Patient(models.Model):
    """Patient Data"""
    patient_id = models.UUIDField(
        primary_key=True,
        default=uuid.uuid4,
        editable=False,
        help_text="Unique patient identifier"
    )

    first_name = models.CharField(max_length=100)
    last_name = models.CharField(max_length=100)
    date_of_birth = models.DateField()

    # Metadata
    created_at = models.DateTimeField(auto_now_add=True)
    updated_at = models.DateTimeField(auto_now=True)

    class Meta:
        ordering = ['last_name', 'first_name']

    def __str__(self):
        return f"{self.patient_id} - {self.first_name} {self.last_name}"

    @property
    def full_name(self):
        return f"{self.first_name} {self.last_name}"

    @property
    def age(self):
        from datetime import date
        today = date.today()
        return today.year - self.date_of_birth.year - (
                (today.month, today.day) < (self.date_of_birth.month, self.date_of_birth.day)
        )


class Session(models.Model):
    """Each time a user connects and records = a new session"""
    session_id = models.UUIDField(
        primary_key=True,
        default=uuid.uuid4,
        editable=False,
        help_text="Unique session identifier"
    )

    patient = models.ForeignKey(
        Patient,
        on_delete=models.CASCADE,
        related_name='sessions'
    )

    # session metadata
    started_at = models.DateTimeField(auto_now_add=True)
    ended_at = models.DateTimeField(null=True, blank=True)

    # optional context
    session_name = models.CharField(max_length=200, blank=True)
    notes = models.TextField(blank=True)
    duration_seconds = models.IntegerField(null=True, blank=True)

    class Meta:
        ordering = ['-started_at']

    def __str__(self):
        return f"Session {self.session_id} - {self.patient.first_name} {self.patient.last_name} at {self.started_at}"

    def end_session(self):
        """Call this when the user stops recording"""
        self.ended_at = timezone.now()
        self.duration_seconds = (self.ended_at - self.started_at).total_seconds()
        self.save()

    @property
    def reading_count(self):
        return self.readings.count()


class Reading(models.Model):
    """Each reading is tied to a session"""
    session = models.ForeignKey(
        Session,
        on_delete=models.CASCADE,
        related_name='readings'
    )

    reading_type = models.CharField(
        max_length=50,
        choices=[
            ('bvp', 'Blood Volume Pulse'),
            ('temperature', 'Temperature (°C)'),
            ('eda', 'Electrodermal Activity (µS)'),
            ('acc', 'Accelerometer (3-axis XYZ)'),
        ]
    )
    value = models.TextField()  # Changed from FloatField to TextField!
    timestamp = models.DateTimeField(auto_now_add=True)

    class Meta:
        ordering = ['timestamp']
        indexes = [
            models.Index(fields=['session', 'reading_type', 'timestamp']),
        ]

    def __str__(self):
        return f"{self.get_reading_type_display()}: {self.value} at {self.timestamp}"
