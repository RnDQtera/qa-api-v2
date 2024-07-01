<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>update assignment request data</name>
   <tag></tag>
   <elementGuidId>13523d93-cae9-4996-b339-21d5e9d87d06</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6IjY0NTRjNDI1M2RkMTAxN2YyODY4OTU3MiIsImNvbXBhbnkiOiI2NWVhNzhhYmY0NTgyNTc4NWI2OGEyZDciLCJjb21wYW55Q29kZSI6IjAwMHYyIiwic2Vzc2lvbiI6IjY1ZWE4M2JjZGRhNWY4OTI4MDA5MTJmNyIsImlhdCI6MTcwOTg2Nzk2NX0.55us-qRS19o--2GYcST2LSUN1MzZsZMBQNGyZuI1d5M</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;destinationGroup\&quot;: \&quot;65f9344de4f427fe9c7f064b\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>4810b604-7bb0-463b-a2e9-26531dc7ccfc</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>619bfb85-fda2-4486-9183-735bbfb282bc</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6IjY0NTRjNDI1M2RkMTAxN2YyODY4OTU3MiIsImNvbXBhbnkiOiI2NWVhNzhhYmY0NTgyNTc4NWI2OGEyZDciLCJjb21wYW55Q29kZSI6IjAwMHYyIiwic2Vzc2lvbiI6IjY1ZWE4M2JjZGRhNWY4OTI4MDA5MTJmNyIsImlhdCI6MTcwOTg2Nzk2NX0.55us-qRS19o--2GYcST2LSUN1MzZsZMBQNGyZuI1d5M</value>
      <webElementGuid>9133dc57-3293-409b-82f8-dfdabb52ffb9</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.4.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${tfUrl}${prl}request/667e908395e5703e3c505a0c</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.TRANSFER_BASE_URL</defaultValue>
      <description></description>
      <id>80ccdbb9-a187-43ac-9b5b-a4e2387d946c</id>
      <masked>false</masked>
      <name>tfUrl</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.prelistEndpoint</defaultValue>
      <description></description>
      <id>7a2f2bd3-fb50-4fd9-ad63-853d0f86418c</id>
      <masked>false</masked>
      <name>prl</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Bearer</defaultValue>
      <description></description>
      <id>e744d489-6174-45ed-b52a-171165468ff7</id>
      <masked>false</masked>
      <name>bt</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
